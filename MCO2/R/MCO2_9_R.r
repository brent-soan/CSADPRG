#***************************************
#Last name : Tan
#Language : R
#Paradigm : Functional
#***************************************


# Required Libraries
library(dplyr)
library(jsonlite)

# Global Variables
projects <- data.frame()

# UTILITY FUNCTIONS

# Load Data
load_data <- function() {
  tryCatch({
    # Load the CSV file
    data <- read.csv("D:/Code/CSADPRG MCO2/dpwh_flood_control_projects.csv", 
                     stringsAsFactors = FALSE)
    
    #Filter data for 2021-2023
    projects <<- data %>%
      filter(FundingYear >= 2021 & FundingYear <= 2023) %>%
      mutate(
        ApprovedBudget = as.numeric(gsub(",", "", ApprovedBudgetForContract)),
        ContractCostNum = as.numeric(gsub(",", "", ContractCost)),
        StartDate = as.Date(StartDate),
        ActualCompletionDate = as.Date(ActualCompletionDate),
        CostSavings = ApprovedBudget - ContractCostNum,
        CompletionDelayDays = as.numeric(difftime(ActualCompletionDate, StartDate, units = "days"))
      ) %>%
      filter(!is.na(ApprovedBudget) & !is.na(ContractCostNum) & !is.na(CompletionDelayDays))
    
    cat(sprintf("\n✓ Successfully loaded %d projects\n", nrow(projects)))
    return(TRUE)
  }, error = function(e) {
    cat(sprintf("\n✗ Error loading data: %s\n", e$message))
    return(FALSE)
  })
}

# Report 1: Regional Summary
report1 <- function() {
  regional_summary <- projects %>%
    group_by(Region, MainIsland) %>%
    summarize(
      TotalBudget = sum(ApprovedBudget, na.rm = TRUE),
      MedianSavings = median(CostSavings, na.rm = TRUE),
      AvgDelay = mean(CompletionDelayDays, na.rm = TRUE),
      HighDelayPct = sum(CompletionDelayDays > 30, na.rm = TRUE) / n() * 100,
      EfficiencyScore = ifelse(AvgDelay > 0, (MedianSavings / AvgDelay) * 100, 0),
      .groups = "drop"
    ) %>%
    arrange(Region, desc(EfficiencyScore))  # Sort by Region and EfficiencyScore
  
  write.csv(regional_summary, "report1_regional_summary.csv", row.names = FALSE)
  cat("✓ Report 1 exported to report1_regional_summary.csv\n")
  return(regional_summary)
}

# Report 2: Contractor Ranking
report2 <- function() {
  contractor_ranking <- projects %>%
    group_by(Contractor) %>%
    filter(n() >= 5) %>%
    summarize(
      ProjectCount = n(),
      TotalCost = sum(ContractCostNum, na.rm = TRUE),
      AvgDelay = mean(CompletionDelayDays, na.rm = TRUE),
      TotalSavings = sum(CostSavings, na.rm = TRUE),
      .groups = "drop"
    ) %>%
    mutate(
      ReliabilityIndex = pmax(0, pmin(100, (1 - (AvgDelay / 90)) * (TotalSavings / TotalCost) * 100))
    ) %>%
    arrange(desc(TotalCost)) %>%
    head(15)
  
  contractor_ranking$Rank <- seq_len(nrow(contractor_ranking))
  
  # Adding sorting by Region if necessary:
  contractor_ranking <- contractor_ranking %>%
    arrange(Contractor)  # Sorting contractors in alphabetical order

  write.csv(contractor_ranking, "report2_contractor_ranking.csv", row.names = FALSE)
  cat("✓ Report 2 exported to report2_contractor_ranking.csv\n")
  return(contractor_ranking)
}

# Report 3: Annual Trends
report3 <- function() {
  annual_trends <- projects %>%
    group_by(FundingYear, TypeOfWork, Region) %>%  # Include Region here
    summarize(
      ProjectCount = n(),
      AvgSavings = mean(CostSavings, na.rm = TRUE),
      OverrunRate = sum(CostSavings < 0, na.rm = TRUE) / n() * 100,
      .groups = "drop"
    ) %>%
    arrange(Region, TypeOfWork, FundingYear) %>%  # Sort by Region first, then by TypeOfWork and FundingYear
    group_by(TypeOfWork) %>%
    mutate(
      YoYChange = ifelse(row_number() == 1, 0,
                        (AvgSavings - lag(AvgSavings)) / abs(lag(AvgSavings)) * 100)
    ) %>%
    ungroup()
  
  write.csv(annual_trends, "report3_annual_trends.csv", row.names = FALSE)
  cat("✓ Report 3 exported to report3_annual_trends.csv\n")
  return(annual_trends)
}


# Summary Stats
generate_summary <- function() {
  summary_stats <- list(
    total_projects = nrow(projects),
    total_contractors = length(unique(projects$Contractor)),
    global_avg_delay = round(mean(projects$CompletionDelayDays, na.rm = TRUE), 2),
    total_savings = round(sum(projects$CostSavings, na.rm = TRUE)),
    total_budget = round(sum(projects$ApprovedBudget, na.rm = TRUE))
  )
  
  write_json(summary_stats, "summary.json", pretty = TRUE)
  cat("✓ Summary exported to summary.json\n")
  return(summary_stats)
}

# DISPLAY PREVIEW
display_preview <- function(num, data, title) {
  cat(sprintf("\n═══════════════════════════════════════════════════════════\n"))
  cat(sprintf("Report %d: %s\n", num, title))
  cat(sprintf("═══════════════════════════════════════════════════════════\n"))
  
  if (nrow(data) == 0) {
    cat("No data to display.\n")
  } else {
    print(head(data, 10))
    cat(sprintf("\n(Showing first 10 rows. Full table: %d rows)\n", nrow(data)))
  }
}

# Main Program
main <- function() {
  cat("\n")
  cat("╔══════════════════════════════════════════════════════════╗\n")
  cat("║   DPWH FLOOD CONTROL DATA ANALYSIS PIPELINE             ║\n")
  cat("╚══════════════════════════════════════════════════════════╝\n")
  
  while (TRUE) {
    cat("\n┌──────────────────────────────────────────────────────────┐\n")
    cat("│ MAIN MENU                                                │\n")
    cat("├──────────────────────────────────────────────────────────┤\n")
    cat("│ [1] Load Data File                                       │\n")
    cat("│ [2] Generate All Reports                                 │\n")
    cat("│ [3] Exit                                                 │\n")
    cat("└──────────────────────────────────────────────────────────┘\n")
    
    choice <- readline(prompt = "\nEnter choice (1-3): ")
    
    if (choice == '1') {
      load_data()
      
    } else if (choice == '2') {
      if (nrow(projects) == 0) {
        cat("\n✗ Error: Please load the data file first (option 1).\n")
        next
      }
      
      cat("\n⟳ Generating reports...\n\n")
      
      r1 <- report1()
      r2 <- report2()
      r3 <- report3()
      summary <- generate_summary()
      
      display_preview(1, r1, "Regional Flood Mitigation Efficiency Summary")
      display_preview(2, r2, "Top Contractors Performance Ranking (>=5 Projects)")
      display_preview(3, r3, "Annual Project Type Cost Overrun Trends")
      
      cat("\n═══════════════════════════════════════════════════════════\n")
      cat("Summary Statistics\n")
      cat("═══════════════════════════════════════════════════════════\n")
      cat(sprintf("Total Projects:       %d\n", summary$total_projects))
      cat(sprintf("Total Contractors:    %d\n", summary$total_contractors))
      cat(sprintf("Avg Delay (days):     %.2f\n", summary$global_avg_delay))
      cat(sprintf("Total Savings:        ₱%.2f M\n", summary$total_savings / 1e6))
      cat(sprintf("Total Budget:         ₱%.2f M\n", summary$total_budget / 1e6))
      
      cat("\n\nAll reports generated successfully!\n")
      cat("\nReturn to Main Menu? (Y/N): ")
      continue_choice <- readline()
      if (tolower(continue_choice) != "y") {
        break
      }
      
    } else if (choice == '3') {
      cat("\n")
      cat("╔══════════════════════════════════════════════════════════╗\n")
      cat("║   Thank you for using the Analysis Pipeline!            ║\n")
      cat("╚══════════════════════════════════════════════════════════╝\n\n")
      break
      
    } else {
      cat("\n✗ Invalid choice. Please enter 1, 2, or 3.\n")
    }
  }
}

# Run the main program
main()