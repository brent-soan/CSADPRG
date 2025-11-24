class CurrencyManager {
    private val rates = mutableMapOf(
        "PHP" to 1.0,
        "USD" to 52.0,
        "JPY" to 0.37,
        "GBP" to 65.0,
        "EUR" to 56.0,
        "CNY" to 7.5
    )

    private val currencyList = listOf("PHP", "USD", "JPY", "GBP", "EUR", "CNY")

    // ========== RECORD EXCHANGE RATE ==========
    fun recordExchangeRate() {
        println()
        println("Record Exchange Rate")
        println()
        for ((i, currency) in currencyList.withIndex()) {
            println("[${i + 1}] $currency")
        }

        print("\nSelect Foreign Currency: ")
        val choice = readln().toIntOrNull()
        val selected = currencyList.getOrNull(choice?.minus(1) ?: -1)

        if (selected == null) {
            println("Invalid selection.")
            return
        }

        print("Exchange Rate: ")
        val rate = readln().toDoubleOrNull()
        if (rate == null) {
            println("Invalid input.")
            return
        }

        rates[selected] = rate
        println("Updated $selected rate: $rate")

        // Back to Main Menu
        while (true) {
            print("\nBack to the Main Menu (Y/N): ")
            val response = readln().trim().uppercase()
            if (response == "Y") return
        }
    }

    // ========== CURRENCY EXCHANGE ==========
    fun exchangeCurrency() {
        println()
        println("Foreign Currency Exchange")
        println("Source Currency Option:")
        for ((i, currency) in currencyList.withIndex()) {
            println("[${i + 1}] $currency")
        }

        print("\nSource Currency: ")
        val sourceChoice = readln().toIntOrNull()
        val sourceCurrency = currencyList.getOrNull(sourceChoice?.minus(1) ?: -1)
        if (sourceCurrency == null) {
            println("Invalid selection.")
            return
        }

        print("Source Amount: ")
        val amount = readln().toDoubleOrNull()
        if (amount == null) {
            println("Invalid input.")
            return
        }

        println("\nExchanged Currency Options:")
        for ((i, currency) in currencyList.withIndex()) {
            println("[${i + 1}] $currency")
        }

        print("\nExchange Currency: ")
        val targetChoice = readln().toIntOrNull()
        val targetCurrency = currencyList.getOrNull(targetChoice?.minus(1) ?: -1)
        if (targetCurrency == null) {
            println("Invalid selection.")
            return
        }

        val converted = amount * (rates[targetCurrency]!! / rates[sourceCurrency]!!)
        println("Exchange Amount: ${"%.2f".format(converted)}")

        // Option to convert another currency
        while (true) {
            print("\nConvert another currency (Y/N)?: ")
            val again = readln().trim().uppercase()
            if (again == "Y") {
                exchangeCurrency() // recursively call again
                return
            } else if (again == "N") {
                return
            }
        }
    }
    fun showCurrencyList() {
        currencyList.forEachIndexed { i, c -> println("[${i + 1}] $c") }
    }

    fun getCurrencyFromUser(): String {
        val choice = readln().toIntOrNull()
        return currencyList.getOrNull(choice?.minus(1) ?: -1) ?: "PHP"
    }

    fun convertToPHP(amount: Double, fromCurrency: String): Double {
        val rate = rates[fromCurrency] ?: 1.0
        return amount * rate // because PHP = 1.0
    }
}
