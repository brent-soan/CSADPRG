
import org.example.CleanData
import org.example.DataProcess
import java.util.Scanner
import org.jetbrains.kotlinx.dataframe.api.*
import org.jetbrains.kotlinx.dataframe.DataRow
import org.jetbrains.kotlinx.dataframe.io.readCsv
import java.time.LocalDate
import java.time.format.DateTimeFormatter
import java.time.format.DateTimeParseException
import org.jetbrains.kotlinx.dataframe.DataFrame

fun main() {
    val scanner = Scanner(System.`in`)
    var choice: Int? = null

    val rawFile: String = "dpwh_flood_control_projects.csv"

    while (choice != 3) {
        println("Select Language Implementation")
        println("[1] Load the file")
        println("[2] Generate Reports")
        println("[3] Exit")
        print("Enter your choice: ")

        if (scanner.hasNextInt()) {
            choice = scanner.nextInt()

            when (choice) {
                1 -> {
                    val prepare = CleanData(rawFile)
                    val cleanedFile: String = "cleaned_dpwh_flood_control_projects.csv"

                    val process = DataProcess(cleanedFile, rawFile)
                    prepare.prepareData()
                    process.loadFile()
                }
                2 -> {
                    val generate = GenerateReports()
                    println("Report 1: Regional Flood Mitigation Efficiency Summary")
                    println("Regional Flood Mitigation Efficiency Summary")
                    println("Filtered: 2021-2023 Projects")

                    generate.generateReport1()
                    println()
                    println("Report 2: Top Contractors Performance Ranking")
                    println("Top Contractors Performance Ranking")
                    println("Top 15 by TotalCost, >= 5 Projects")
                    generate.generateReport2()

                    println()
                    println("Report 3: Annual Project Type Cost Overrun Trends")
                    println("Annual Project Type Cost Overrun Trends")
                    println("Grouped by FundingYear and TypeOfWork")
                    generate.generateReport3()

                    println()
                    println("Summary stats: summary.json")
                    generate.produceSummary()
                    generate.printSummary()
                }
                3 -> {
                    println("Exiting the program. Goodbye!")
                }
                else -> {
                    println("Invalid choice. Please enter a number between 1 and 3.")
                }
            }
        } else {
            println("Invalid input. Please enter a number.")
            scanner.next()
        }
        println("\n")
    }
    scanner.close()
}
