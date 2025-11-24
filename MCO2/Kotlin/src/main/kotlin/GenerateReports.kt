import org.jetbrains.kotlinx.dataframe.api.*
import org.jetbrains.kotlinx.dataframe.io.writeCsv
import org.jetbrains.kotlinx.dataframe.DataFrame
import org.jetbrains.kotlinx.dataframe.DataRow
import org.jetbrains.kotlinx.dataframe.io.readCsv
import kotlin.math.max
import kotlin.math.min
import java.text.DecimalFormat
import kotlinx.serialization.json.*
import java.io.File


class GenerateReports(val cleanedFile: String = "cleaned_dpwh_flood_control_projects.csv") {


    private val df = DataFrame.readCsv(cleanedFile)

    // Formatting utility
    object FormatUtil {
        private val df2 = DecimalFormat("#,##0.00")
        fun formatNumber(value: Any?): String {
            return when (value) {
                is Long -> df2.format(value.toDouble())
                is Float -> df2.format(value.toDouble())
                is Double -> df2.format(value)
                null -> ""
                else -> value.toString()
            }
        }
    }

    fun DataFrame<*>.formatForOutput(vararg numericCols: String): DataFrame<*> {
        var newDf = this
        for (col in numericCols) {
            newDf = newDf.convert(col) { value -> FormatUtil.formatNumber(value) }
        }
        return newDf
    }

    fun DataFrame<*>.prettyPrint(maxRows: Int = 20) {
        val colWidths = this.columns().associate { col ->
            val name = col.name()
            val maxCell = this.rows().take(maxRows)
                .map { FormatUtil.formatNumber(it[col]) }
                .maxOf { it.length }
            name to maxOf(name.length, maxCell) + 2
        }

        // Header
        colWidths.forEach { (name, width) -> print(name.padEnd(width)) }
        println()
        colWidths.values.forEach { width -> print("-".repeat(width)) }
        println()

        // Rows
        this.rows().take(maxRows).forEach { row ->
            colWidths.forEach { (name, width) ->
                val formatted = FormatUtil.formatNumber(row[name])
                print(formatted.padEnd(width))
            }
            println()
        }

        if (this.rowsCount() > maxRows) {
            println("... (${this.rowsCount() - maxRows} more rows)")
        }
    }

    // ----------------------- REPORT 1 -----------------------
    fun generateReport1(): DataFrame<*> {

        // Group and compute basic aggregates
        val grouped = df.groupBy("Region", "MainIsland").aggregate {
            sum("ApprovedBudgetForContract") into "TotalApprovedBudget"
            median("CostSavings") into "MedianSavings"
            mean("CompletionDelayDays") into "AvgDelayDays"
        }

        // Add DelayOver30Percent and RawEfficiency
        val scoredRaw = grouped
            .add("DelayOver30Percent") { row ->
                val groupRows = df.filter { it["Region"] == row["Region"] && it["MainIsland"] == row["MainIsland"] }
                val delays = groupRows.map { (it["CompletionDelayDays"] as Number).toInt() }
                if (delays.isEmpty()) 0.0
                else delays.count { it > 30 }.toDouble() / delays.size * 100
            }
            .add("RawEfficiency") {
                val median = this["MedianSavings"] as Double
                val avgDelay = this["AvgDelayDays"] as Double
                if (avgDelay <= 0) 0.0 else (median / avgDelay)
            }

        // Normalize EfficiencyScore to 0–100
        val maxEfficiency = scoredRaw["RawEfficiency"].values()
            .map { it as? Double ?: 0.0 } // safely cast to Double
            .maxOrNull() ?: 1.0

        val scored = scoredRaw.add("EfficiencyScore") { row ->
            val raw = row["RawEfficiency"] as Double
            min(100.0, (raw / maxEfficiency) * 100)
        }


        // Sort by EfficiencyScore descending
        val finalReport = scored.sortByDesc("EfficiencyScore")

        // Select relevant columns
        val output = finalReport.select(
            "Region",
            "MainIsland",
            "TotalApprovedBudget",
            "MedianSavings",
            "AvgDelayDays",
            "DelayOver30Percent",
            "EfficiencyScore"
        )

        // Format numeric columns for output
        val standardized = output.formatForOutput(
            "TotalApprovedBudget",
            "MedianSavings",
            "AvgDelayDays",
            "DelayOver30Percent",
            "EfficiencyScore"
        )

        // Print first 3 rows
        standardized.prettyPrint(3)

        // Write to CSV
        standardized.writeCsv("report1_regional_efficiency.csv")

        return standardized
    }

    // ----------------------- REPORT 2 -----------------------
    fun generateReport2(): DataFrame<*> {
        val grouped = df.groupBy("Contractor").aggregate {
            count() into "NumProjects"
            mean("CompletionDelayDays") into "AvgDelayDays"
            sum("CostSavings") into "TotalCostSavings"
            sum("ContractCost") into "TotalContractCost"
        }

        val filtered = grouped.filter { it["NumProjects"] as Int >= 5 }

        val scored = filtered
            .add("ReliabilityIndex") { row ->
                val avgDelay = row["AvgDelayDays"] as? Double ?: 0.0
                val totalSavings = row["TotalCostSavings"] as? Double ?: 0.0
                val totalCost = row["TotalContractCost"] as? Double ?: 1.0 // avoid div by zero
                val raw = (1 - (avgDelay / 90.0)) * (totalSavings / totalCost) * 100
                max(0.0, min(100.0, raw))
            }
            .add("RiskFlag") { row ->
                val reliability = row["ReliabilityIndex"] as? Double ?: 0.0
                if (reliability < 50.0) "High Risk" else ""
            }

        val finalReport = scored.sortByDesc("ReliabilityIndex").take(15)

        val standardized = finalReport.formatForOutput(
            "AvgDelayDays",
            "TotalCostSavings",
            "TotalContractCost",
            "ReliabilityIndex"
        )

        standardized.prettyPrint(5)
        standardized.writeCsv("report2_top_contractors.csv")
        return standardized
    }

    // ----------------------- REPORT 3 -----------------------
    fun generateReport3(): DataFrame<*> {
        // Step 1: Aggregate basic metrics
        val grouped = df.groupBy("FundingYear", "TypeOfWork").aggregate {
            count() into "TotalProjects"
            mean("CostSavings") into "AvgSavings"
        }

        // Step 2: Compute OverrunRatePercent
        val withOverrun = grouped.add("OverrunRatePercent") { row ->
            val groupRows = df.filter {
                it["FundingYear"] == row["FundingYear"] &&
                        it["TypeOfWork"] == row["TypeOfWork"]
            }
            val negativeCount = groupRows.count { (it["CostSavings"] as? Double ?: 0.0) < 0.0 }
            val total = groupRows.count()
            if (total == 0) 0.0 else negativeCount.toDouble() / total * 100
        }

        // Step 3: Compute YoYChangePercent (2021 baseline)
        val savings2021 = withOverrun.filter { it["FundingYear"] == 2021 }
            .associate { it["TypeOfWork"] to (it["AvgSavings"] as Double) }

        val withYoY = withOverrun.add("YoYChangePercent") { row ->
            val type = row["TypeOfWork"]
            val year = row["FundingYear"] as Int
            val baseline = savings2021[type] ?: return@add 0.0
            if (year == 2021) 0.0 else ((row["AvgSavings"] as Double - baseline) / baseline) * 100
        }

        val finalReport = withYoY.sortBy("FundingYear").sortByDesc("AvgSavings")

        val standardized = finalReport.formatForOutput(
            "AvgSavings",
            "OverrunRatePercent",
            "YoYChangePercent"
        )

        standardized.prettyPrint(5)
        standardized.writeCsv("report3_annual_project_type.csv")
        return standardized
    }

    fun produceSummary(): JsonObject {
        // Decimal formatter for 2 decimals
        val df2 = DecimalFormat("#.##")

        // Total number of projects
        val totalProjects = df.rowsCount()

        // Total number of unique contractors
        val totalContractors = df["Contractor"].distinct().count()

        // Total number of unique provinces with projects
        val totalProvinces = df["Province"].distinct().count()

        // Global average delay
        val avgDelayRaw = df["CompletionDelayDays"].values()
            .mapNotNull { (it as? Number)?.toDouble() }
            .average()
        val avgDelayRounded = df2.format(avgDelayRaw).toDouble() // 2 decimal places

        // Total CostSavings
        val totalSavings = df["CostSavings"].values()
            .mapNotNull { (it as? Number)?.toDouble() }
            .sum()

        // Build JSON object
        val summary = buildJsonObject {
            put("TotalProjects", totalProjects)
            put("TotalContractors", totalContractors)
            put("TotalProvinces", totalProvinces)
            put("GlobalAvgDelay", avgDelayRounded)
            put("TotalSavings", totalSavings)
        }

        // Write JSON to file
        File("summary.json").writeText(summary.toString())

        return summary
    }

    fun printSummary() {
        val jsonFile = File("summary.json")
        if (!jsonFile.exists()) {
            println("summary.json not found. Please run produceSummary() first.")
            return
        }

        val jsonString = jsonFile.readText()
        val jsonObject = Json.parseToJsonElement(jsonString).jsonObject

        // Safely cast to JsonPrimitive to get double values
        val globalAvgDelay = (jsonObject["GlobalAvgDelay"] as? JsonPrimitive)?.doubleOrNull ?: 0.0
        val totalSavings = (jsonObject["TotalSavings"] as? JsonPrimitive)?.doubleOrNull ?: 0.0

        // Format numbers
        val totalSavingsFormatted = DecimalFormat("#,##0.00").format(totalSavings)

        println("Global Average Delay: %.2f days".format(globalAvgDelay))
        println("Total Savings: $totalSavingsFormatted")
    }

}
