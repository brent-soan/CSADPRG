class AccountManager(private val currencyManager: CurrencyManager) {
    private var accountName: String? = null
    private var balance: Double = 0.0
    private val interestRate: Double = 0.05 // 5% per annum
    private val currency: String = "PHP"    // Default currency

    fun getName(): String? {
        return accountName
    }

    // ========== REGISTER ACCOUNT ==========
    fun registerAccount() {
        println()
        println("Register Account Name")
        print("Enter Account Name: ")
        accountName = readln()
        println("Account registered: $accountName")

        backToMenuPrompt()
    }

    // ========== DISPLAY ACCOUNT ==========
    fun displayAccount() {
        println()
        println("Register Account Name")
        println("Account Name: $accountName")

        backToMenuPrompt()
    }

    // ========== DEPOSIT ==========
    fun depositAmount() {
        // choose currency
        println("Select currency:")
        currencyManager.showCurrencyList()
        val currency = currencyManager.getCurrencyFromUser()

        // deposit amount
        print("Deposit Amount ($currency): ")
        val amount = readln().toDoubleOrNull() ?: return println("Invalid input.")

        // convert to PHP
        val amountInPHP = currencyManager.convertToPHP(amount, currency)
        balance += amountInPHP

        println("Converted to PHP: ${"%.2f".format(amountInPHP)}")
        println("Updated Balance: ${"%.2f".format(balance)} PHP")
        backToMenuPrompt()
    }

    // ========== WITHDRAW ==========
    fun withdrawAmount() {
        if (accountName == null) {
            println("Account not found. Please Register First.")
            return
        }

        println()
        println("Withdraw Amount")
        println("Account Name: $accountName")
        println("Current Balance: ${"%.2f".format(balance)} $currency")
        println("Currency: $currency")
        println()

        // Choose withdrawal currency
        println("Select currency to withdraw:")
        currencyManager.showCurrencyList()
        val currency = currencyManager.getCurrencyFromUser()

        print("Withdraw Amount ($currency): ")
        val amount = readln().toDoubleOrNull()
        if (amount == null || amount <= 0) {
            println("Invalid input.")
            return
        }

        // Convert withdrawal to PHP equivalent
        val amountInPHP = currencyManager.convertToPHP(amount, currency)

        if (amountInPHP > balance) {
            println("Insufficient balance.")
            return
        }

        balance -= amountInPHP
        println("Converted to PHP: ${"%.2f".format(amountInPHP)}")
        println("Updated Balance: ${"%.2f".format(balance)} PHP")

        backToMenuPrompt()
    }

    // ========== SHOW INTEREST ==========
    fun showInterest() {
        if (accountName == null) {
            println("No account found. Please register first.")
            return
        }

        println()
        println("Show Interest Amount")
        println("Account Name: $accountName")
        println("Current Balance: ${"%.2f".format(balance)} $currency")
        println("Currency: $currency")
        println("Interest Rate: 5% per annum")

        print("\nTotal Number of Days: ")
        val days = readln().toIntOrNull()
        if (days == null || days <= 0) {
            println("Invalid input.")
            return
        }

        var tempBalance = balance
        println("\nDay | Interest | Balance")
        for (i in 1..days) {
            val dailyInterest = tempBalance * (interestRate / 365)
            tempBalance += dailyInterest
            println(
                "%-4d| %-9s| %-9s".format(
                    i,
                    "%.2f".format(dailyInterest),
                    "%.2f".format(tempBalance)
                )
            )
        }

        backToMenuPrompt()
    }

    // ========== HELPER ==========
    private fun backToMenuPrompt() {
        while (true) {
            print("\nBack to the Main Menu (Y/N): ")
            val choice = readln().trim().uppercase()
            if (choice == "Y") return
        }
    }
}
