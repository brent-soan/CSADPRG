class BankingApp {
    private val currencyManager = CurrencyManager()
    private val accountManager = AccountManager(currencyManager)

    fun run() {
        while (true) {
            println()
            println("Select Transaction:")
            println("[1] Register Account Name")
            println("[2] Deposit Amount")
            println("[3] Withdraw Amount")
            println("[4] Currency Exchange")
            println("[5] Record Exchange Rates")
            println("[6] Show Interest Amount")
            println("[0] Exit")

            print("Enter choice: ")
            when(readln().toIntOrNull()) {
                1 -> if (accountManager.getName() == null)
                    accountManager.registerAccount()
                else accountManager.displayAccount()
                2 -> accountManager.depositAmount()
                3 -> accountManager.withdrawAmount()
                4 -> currencyManager.exchangeCurrency()
                5 -> currencyManager.recordExchangeRate()
                6 -> accountManager.showInterest()
                0 -> return
                else -> println("Invalid option.")
            }
        }
    }
}
