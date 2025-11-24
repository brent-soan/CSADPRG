package org.example

import org.jetbrains.kotlinx.dataframe.api.*
import org.jetbrains.kotlinx.dataframe.DataFrame
import org.jetbrains.kotlinx.dataframe.io.readCsv
import org.jetbrains.kotlinx.dataframe.io.writeCsv
import java.time.LocalDate
import java.time.format.DateTimeParseException
import java.time.temporal.ChronoUnit

class CleanData(val file: String = "dpwh_flood_control_projects.csv") {

    private val df = DataFrame.readCsv(file)

    /**
     * Prepares the dataset for analysis:
     * - Converts financial fields to floats
     * - Parses date columns
     * - Computes CostSavings
     * - Computes CompletionDelayDays
     * - Filters projects from 2021-2023
     */
    fun prepareData(): DataFrame<*> {
        var dfPrepared = df

        // Convert financial fields to floats
        val financialColumns = listOf("ApprovedBudgetForContract", "ContractCost")
        for (col in financialColumns) {
            dfPrepared = dfPrepared.convert(col) { value ->
                value.toString().replace(",", "").toFloatOrNull() ?: 0f
            }
        }


        val dateColumns = listOf("StartDate", "ActualCompletionDate")
        for (col in dateColumns) {
            dfPrepared = dfPrepared.convert(col) { value ->
                try {
                    LocalDate.parse(value.toString())
                } catch (e: DateTimeParseException) {
                    null
                }
            }
        }


        // Compute CostSavings
        dfPrepared = dfPrepared.add("CostSavings") { row ->
            val approved = row["ApprovedBudgetForContract"] as Float
            val cost = row["ContractCost"] as Float
            approved - cost
        }

        // Compute CompletionDelayDays
        dfPrepared = dfPrepared.add("CompletionDelayDays") { row ->
            val start = row["StartDate"] as? LocalDate
            val actual = row["ActualCompletionDate"] as? LocalDate
            if (start != null && actual != null) {
                val days = ChronoUnit.DAYS.between(start, actual)
                if (days > 0) days.toInt() else 0
            } else {
                0
            }
        }

        // Filter projects from 2021-2023
        dfPrepared = dfPrepared.filter { row ->
            val year = (row["StartDate"] as? LocalDate)?.year
            year != null && year in 2021..2023
        }

        dfPrepared.writeCsv("cleaned_dpwh_flood_control_projects.csv")
        return dfPrepared
    }
}
