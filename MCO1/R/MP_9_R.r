# ================================
# Last name: Tan
# Language: R
# Paradigm(s) : OOP, functional
# ================================

#initializing
app <- list(
  accountName = "",
  balance = 0,
  currency = "PHP",
  rates = list(
    PHP = 1,
    USD = 56.50,
    JPY = 0.38,
    GBP = 72.50,
    EUR = 61.20,
    CNY = 7.85
  )
)

currencies <- c("PHP", "USD", "JPY", "GBP", "EUR", "CNY")

#Clear screen function
clear_screen <- function() {
  cat("\033[2J\033[H") 
}

#Main Menu Function
showMainMenu <- function() {
  clear_screen()
  cat("==========================\n")
  cat("  BANKING APPLICATION\n")
  cat("==========================\n\n")
  cat("Select Transaction:\n")
  cat("[1] Register Account Name\n")
  cat("[2] Deposit Amount\n")
  cat("[3] Withdraw Amount\n")
  cat("[4] Currency Exchange\n")
  cat("[5] Record Exchange Rates\n")
  cat("[6] Show Interest Amount\n")
  cat("[0] Exit\n\n")

  choice <- readline("Enter choice: ")

  if (choice == "1") registerAccount()
  else if (choice == "2") depositAmount()
  else if (choice == "3") withdrawAmount()
  else if (choice == "4") currencyExchange()
  else if (choice == "5") recordExchangeRate()
  else if (choice == "6") showInterest()
  else if (choice == "0") {
    cat("\nThank you for using our banking application!\n")
    quit(save = "no")
  } else {
    cat("\nInvalid choice!\n")
    readline("Press Enter to continue...")
    showMainMenu()
  }
}

#Register Account
registerAccount <- function() {
  clear_screen()
  cat("==========================\n")
  cat("  REGISTER ACCOUNT NAME\n")
  cat("==========================\n\n")
  
  app$accountName <<- readline("Account Name: ")
  cat("\nAccount registered successfully:", app$accountName, "\n")
  
  readline("\nBack to Main Menu (Y/N)? ")
  showMainMenu()
}

#Deposit Amount
depositAmount <- function() {
  clear_screen()
  cat("==========================\n")
  cat("     DEPOSIT AMOUNT\n")
  cat("==========================\n\n")
  
  if (app$accountName == "") {
    cat("Please register an account first!\n")
    readline("\nPress Enter to continue...")
    return(showMainMenu())
  }

  cat("Account Name:", app$accountName, "\n")
  cat("Current Balance:", sprintf("%.2f", app$balance), app$currency, "\n\n")

  cat("Select Currency:\n")
  for (i in seq_along(currencies)) cat("[", i, "] ", currencies[i], "\n")
  cat("\n")

  currChoice <- as.integer(readline("Currency: "))
  selected <- currencies[currChoice]
  
  if (is.na(selected)) {
    cat("Invalid currency!\n")
    readline("\nPress Enter to continue...")
    return(depositAmount())
  }

  amount <- as.numeric(readline("Deposit Amount: "))

  if (is.na(amount) || amount <= 0) {
    cat("Invalid amount!\n")
    readline("\nPress Enter to continue...")
    return(depositAmount())
  }

  phpAmount <- amount * app$rates[[selected]]
  app$balance <<- app$balance + phpAmount

  cat("\nDeposit successful!\n")
  cat("Updated Balance:", sprintf("%.2f", app$balance), "PHP\n")

  readline("\nBack to Main Menu (Y/N)? ")
  showMainMenu()
}

#Withdraw Amount
withdrawAmount <- function() {
  clear_screen()
  cat("=================================\n")
  cat("     WITHDRAW AMOUNT\n")
  cat("=================================\n\n")
  
  if (app$accountName == "") {
    cat("Please register an account first!\n")
    readline("\nPress Enter to continue...")
    return(showMainMenu())
  }

  cat("Account Name:", app$accountName, "\n")
  cat("Current Balance:", sprintf("%.2f", app$balance), app$currency, "\n\n")

  cat("Select Currency:\n")
  for (i in seq_along(currencies)) cat("[", i, "] ", currencies[i], "\n")
  cat("\n")

  currChoice <- as.integer(readline("Currency: "))
  selected <- currencies[currChoice]
  
  if (is.na(selected)) {
    cat("Invalid currency!\n")
    readline("\nPress Enter to continue...")
    return(withdrawAmount())
  }

  amount <- as.numeric(readline("Withdraw Amount: "))

  if (is.na(amount) || amount <= 0) {
    cat("Invalid amount!\n")
    readline("\nPress Enter to continue...")
    return(withdrawAmount())
  }

  phpAmount <- amount * app$rates[[selected]]
  
  if (phpAmount > app$balance) {
    cat("\nInsufficient balance!\n")
    readline("\nPress Enter to continue...")
    return(withdrawAmount())
  }

  app$balance <<- app$balance - phpAmount
  cat("\nWithdrawal successful!\n")
  cat("Updated Balance:", sprintf("%.2f", app$balance), "PHP\n")

  readline("\nBack to Main Menu (Y/N)? ")
  showMainMenu()
}

#Record Exchange Rate
recordExchangeRate <- function() {
  clear_screen()
  cat("=================================\n")
  cat("   RECORD EXCHANGE RATE\n")
  cat("=================================\n\n")

  for (i in seq_along(currencies)) cat("[", i, "] ", currencies[i], "\n")
  cat("\n")

  choice <- as.integer(readline("Select Foreign Currency: "))
  selected <- currencies[choice]
  
  if (is.na(selected)) {
    cat("Invalid currency!\n")
    readline("\nPress Enter to continue...")
    return(recordExchangeRate())
  }

  if (selected == "PHP") {
    cat("\nPHP is the base currency. Rate is always 1.00\n")
    readline("\nPress Enter to continue...")
    return(recordExchangeRate())
  }

  rate <- as.numeric(readline("Exchange Rate (to PHP): "))
  
  if (is.na(rate) || rate <= 0) {
    cat("Invalid rate!\n")
    readline("\nPress Enter to continue...")
    return(recordExchangeRate())
  }

  app$rates[[selected]] <<- rate
  cat("\nExchange rate for", selected, "set to", sprintf("%.2f", rate), "PHP\n")

  readline("\nBack to Main Menu (Y/N)? ")
  showMainMenu()
}

#Currency Exchange
currencyExchange <- function() {
  clear_screen()
  cat("=================================\n")
  cat("   FOREIGN CURRENCY EXCHANGE\n")
  cat("=================================\n\n")

  cat("Source Currency Option:\n")
  for (i in seq_along(currencies)) cat("[", i, "] ", currencies[i], "\n")
  cat("\n")

  fromChoice <- as.integer(readline("Source Currency: "))
  fromCurrency <- currencies[fromChoice]

  if (is.na(fromCurrency)) {
    cat("Invalid currency!\n")
    readline("\nPress Enter to continue...")
    return(currencyExchange())
  }

  amount <- as.numeric(readline("Source Amount: "))
  if (is.na(amount) || amount <= 0) {
    cat("Invalid amount!\n")
    readline("\nPress Enter to continue...")
    return(currencyExchange())
  }

  cat("\nExchange Currency Options:\n")
  for (i in seq_along(currencies)) cat("[", i, "] ", currencies[i], "\n")
  cat("\n")

  toChoice <- as.integer(readline("Exchange Currency: "))
  toCurrency <- currencies[toChoice]

  if (is.na(toCurrency)) {
    cat("Invalid currency!\n")
    readline("\nPress Enter to continue...")
    return(currencyExchange())
  }

  phpAmount <- amount * app$rates[[fromCurrency]]
  result <- phpAmount / app$rates[[toCurrency]]

  cat("\nExchange Result:\n")
  cat(sprintf("%.2f %s = %.2f %s\n", amount, fromCurrency, result, toCurrency))

  another <- readline("\nConvert another currency (Y/N)? ")
  if (toupper(another) == "Y") currencyExchange()
  else showMainMenu()
}

#Show Interest
showInterest <- function() {
  clear_screen()
  cat("=================================\n")
  cat("   SHOW INTEREST AMOUNT\n")
  cat("=================================\n\n")

  if (app$accountName == "") {
    cat("Please register an account first!\n")
    readline("\nPress Enter to continue...")
    return(showMainMenu())
  }

  cat("Account Name:", app$accountName, "\n")
  cat("Current Balance:", sprintf("%.2f", app$balance), app$currency, "\n")
  cat("Interest Rate: 5% per annum\n\n")

  days <- as.integer(readline("Total Number of Days: "))
  
  if (is.na(days) || days <= 0) {
    cat("Invalid number of days!\n")
    readline("\nPress Enter to continue...")
    return(showInterest())
  }

  cat("\n==================================================\n")
  cat("Day\t|\tInterest\t|\tBalance\n")
  cat("==================================================\n")

  annualRate <- 0.05
  balance <- app$balance
  
  for (i in 1:days) {
    dailyInterest <- app$balance * (annualRate / 365)
    balance <- balance + dailyInterest
    cat(i, "\t|\t", sprintf("%.2f", dailyInterest), "\t\t|\t", sprintf("%.2f", balance), "\n")
  }

  cat("==================================================\n")

  readline("\nBack to Main Menu (Y/N)? ")
  showMainMenu()
}

# --- Run App ---
cat("\nWelcome to Banking Application!\n\n")
showMainMenu()
