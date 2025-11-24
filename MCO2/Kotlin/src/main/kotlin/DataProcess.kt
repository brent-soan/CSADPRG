package org.example

import org.jetbrains.kotlinx.dataframe.api.*
import org.jetbrains.kotlinx.dataframe.DataFrame
import org.jetbrains.kotlinx.dataframe.DataRow
import org.jetbrains.kotlinx.dataframe.io.readCsv
import java.time.LocalDate
import java.time.format.DateTimeFormatter
import java.time.format.DateTimeParseException

class DataProcess(val cleanedFile: String, val rawFile: String) {

    private val formatter = DateTimeFormatter.ofPattern("M/d/yyyy") // match CSV format

    fun loadFile() {
        // Step 1 — load
        val cleanedDf = DataFrame.readCsv(cleanedFile)
        val rawDf = DataFrame.readCsv(rawFile)

        // Step 2 — validate rows
        val (validRows) = cleanedDf.rows().partition { validateRowCleaned(it) }
        val validCleanedDf = validRows.toDataFrame()

        val (validRowsRaw) = rawDf.rows().partition { validateRowRaw(it) }
        val validRawDf = validRowsRaw.toDataFrame()


        println("Processing dataset...(${validRawDf.rowsCount()} valid rows loaded, ${validCleanedDf.rowsCount()} filtered for 2021 - 2023)")
    }

    private fun validateRowCleaned(row: DataRow<*>): Boolean {
        val required = listOf(
            "ApprovedBudgetForContract",
            "ContractCost",
            "FundingYear",
            "Region",
            "Province",
            "LegislativeDistrict",
            "Municipality",
            "DistrictEngineeringOffice",
            "ProjectId",
            "ProjectName",
            "TypeOfWork",
            "ContractId",
            "ActualCompletionDate",
            "Contractor",
            "ContractorCount",
            "StartDate",
            "ProjectLatitude",
            "ProjectLongitude",
            "ProvincialCapital",
            "ProvincialCapitalLatitude",
            "ProvincialCapitalLongitude",
            "CostSavings",
            "CompletionDelayDays"
        )

        if (required.any { row[it] == null }) return false
        if (!isFloat(row["ApprovedBudgetForContract"])) return false
        if (!isFloat(row["ContractCost"])) return false
        if (!isInt(row["FundingYear"])) return false

        return true
    }

    private fun validateRowRaw(row: DataRow<*>): Boolean {
        val required = listOf(
            "ApprovedBudgetForContract",
            "ContractCost",
            "FundingYear",
            "Region",
            "Province",
            "LegislativeDistrict",
            "Municipality",
            "DistrictEngineeringOffice",
            "ProjectId",
            "ProjectName",
            "TypeOfWork",
            "ContractId",
            "ActualCompletionDate",
            "Contractor",
            "ContractorCount",
            "StartDate",
            "ProjectLatitude",
            "ProjectLongitude",
            "ProvincialCapital",
            "ProvincialCapitalLatitude",
            "ProvincialCapitalLongitude",
        )

        if (required.any { row[it] == null }) return false
        if (!isFloat(row["ApprovedBudgetForContract"])) return false
        if (!isFloat(row["ContractCost"])) return false
        if (!isInt(row["FundingYear"])) return false

        return true
    }

    private fun isFloat(value: Any?): Boolean {
        return try {
            value.toString().replace(",", "").toFloat()
            true
        } catch (e: Exception) {
            false
        }
    }

    private fun isInt(value: Any?): Boolean {
        return value.toString().toIntOrNull() != null
    }

    private fun isValidDate(value: String): Boolean {
        return try {
            LocalDate.parse(value, formatter) // <-- use formatter
            true
        } catch (e: DateTimeParseException) {
            false
        }
    }
}
