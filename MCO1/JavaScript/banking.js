const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const app = {
    accountName: '',
    balance: 0,
    currency: 'PHP',
    rates: {
        PHP: 1,
        USD: 56.50,
        JPY: 0.38,
        GBP: 72.50,
        EUR: 61.20,
        CNY: 7.85
    }
};

function showMainMenu() {
    console.clear();
    console.log('==========================');
    console.log('  BANKING APPLICATION');
    console.log('==========================\n');
    console.log('Select Transaction:');
    console.log('[1] Register Account Name');
    console.log('[2] Deposit Amount');
    console.log('[3] Withdraw Amount');
    console.log('[4] Currency Exchange');
    console.log('[5] Record Exchange Rates');
    console.log('[6] Show Interest Amount');
    console.log('[0] Exit\n');
    
    rl.question('Enter choice: ', function(choice) {
        if (choice === '1') {
            registerAccount();
        } else if (choice === '2') {
            depositAmount();
        } else if (choice === '3') {
            withdrawAmount();
        } else if (choice === '4') {
            currencyExchange();
        } else if (choice === '5') {
            recordExchangeRate();
        } else if (choice === '6') {
            showInterest();
        } else if (choice === '0') {
            console.log('\nThank you for using our banking application!');
            rl.close();
        } else {
            console.log('\nInvalid choice!');
            rl.question('Press Enter to continue...', function() {
                showMainMenu();
            });
        }
    });
}

function registerAccount() {
    console.clear();
    console.log('==========================');
    console.log('  REGISTER ACCOUNT NAME');
    console.log('==========================\n');
    
    rl.question('Account Name: ', function(name) {
        app.accountName = name.trim();
        console.log('\nAccount registered successfully: ' + app.accountName);
        
        rl.question('\nBack to Main Menu (Y/N)? ', function(back) {
            showMainMenu();
        });
    });
}

function depositAmount() {
    console.clear();
    console.log('==========================');
    console.log('       DEPOSIT AMOUNT');
    console.log('==========================\n');
    
    if (!app.accountName) {
        console.log('Please register an account first!');
        rl.question('\nPress Enter to continue...', function() {
            showMainMenu();
        });
        return;
    }
    
    console.log('Account Name: ' + app.accountName);
    console.log('Current Balance: ' + app.balance.toFixed(2) + ' ' + app.currency + '\n');
    
    console.log('Select Currency:');
    console.log('[1] Philippine Peso (PHP)');
    console.log('[2] United States Dollar (USD)');
    console.log('[3] Japanese Yen (JPY)');
    console.log('[4] British Pound Sterling (GBP)');
    console.log('[5] Euro (EUR)');
    console.log('[6] Chinese Yuan Renminbi (CNY)\n');
    
    rl.question('Currency: ', function(currChoice) {
        const currencies = ['PHP', 'USD', 'JPY', 'GBP', 'EUR', 'CNY'];
        const selectedCurrency = currencies[parseInt(currChoice) - 1];
        
        if (!selectedCurrency) {
            console.log('Invalid currency!');
            rl.question('\nPress Enter to continue...', function() {
                depositAmount();
            });
            return;
        }
        
        rl.question('Deposit Amount: ', function(amountStr) {
            const amount = parseFloat(amountStr);
            
            if (isNaN(amount) || amount <= 0) {
                console.log('Invalid amount!');
                rl.question('\nPress Enter to continue...', function() {
                    depositAmount();
                });
                return;
            }
            
            const phpAmount = amount * app.rates[selectedCurrency];
            app.balance = app.balance + phpAmount;
            
            console.log('\nDeposit successful!');
            console.log('Updated Balance: ' + app.balance.toFixed(2) + ' PHP');
            
            rl.question('\nBack to Main Menu (Y/N)? ', function() {
                showMainMenu();
            });
        });
    });
}

function withdrawAmount() {
    console.clear();
    console.log('=================================');
    console.log('   WITHDRAW AMOUNT');
    console.log('=================================\n');
    
    if (!app.accountName) {
        console.log('Please register an account first!');
        rl.question('\nPress Enter to continue...', function() {
            showMainMenu();
        });
        return;
    }
    
    console.log('Account Name: ' + app.accountName);
    console.log('Current Balance: ' + app.balance.toFixed(2) + ' ' + app.currency + '\n');
    
    console.log('Select Currency:');
    console.log('[1] Philippine Peso (PHP)');
    console.log('[2] United States Dollar (USD)');
    console.log('[3] Japanese Yen (JPY)');
    console.log('[4] British Pound Sterling (GBP)');
    console.log('[5] Euro (EUR)');
    console.log('[6] Chinese Yuan Renminbi (CNY)\n');
    
    rl.question('Currency: ', function(currChoice) {
        const currencies = ['PHP', 'USD', 'JPY', 'GBP', 'EUR', 'CNY'];
        const selectedCurrency = currencies[parseInt(currChoice) - 1];
        
        if (!selectedCurrency) {
            console.log('Invalid currency!');
            rl.question('\nPress Enter to continue...', function() {
                withdrawAmount();
            });
            return;
        }
        
        rl.question('Withdraw Amount: ', function(amountStr) {
            const amount = parseFloat(amountStr);
            
            if (isNaN(amount) || amount <= 0) {
                console.log('Invalid amount!');
                rl.question('\nPress Enter to continue...', function() {
                    withdrawAmount();
                });
                return;
            }
            
            // Convert to PHP
            const phpAmount = amount * app.rates[selectedCurrency];
            
            if (phpAmount > app.balance) {
                console.log('\nInsufficient balance!');
                rl.question('\nPress Enter to continue...', function() {
                    withdrawAmount();
                });
                return;
            }
            
            app.balance = app.balance - phpAmount;
            
            console.log('\nWithdrawal successful!');
            console.log('Updated Balance: ' + app.balance.toFixed(2) + ' PHP');
            
            rl.question('\nBack to Main Menu (Y/N)? ', function() {
                showMainMenu();
            });
        });
    });
}

function recordExchangeRate() {
    console.clear();
    console.log('=================================');
    console.log('   RECORD EXCHANGE RATE');
    console.log('=================================\n');
    
    console.log('[1] Philippine Peso (PHP) - Base Currency');
    console.log('[2] United States Dollar (USD)');
    console.log('[3] Japanese Yen (JPY)');
    console.log('[4] British Pound Sterling (GBP)');
    console.log('[5] Euro (EUR)');
    console.log('[6] Chinese Yuan Renminbi (CNY)\n');
    
    rl.question('Select Foreign Currency: ', function(choice) {
        const currencies = ['PHP', 'USD', 'JPY', 'GBP', 'EUR', 'CNY'];
        const selectedCurrency = currencies[parseInt(choice) - 1];
        
        if (!selectedCurrency) {
            console.log('Invalid currency!');
            rl.question('\nPress Enter to continue...', function() {
                recordExchangeRate();
            });
            return;
        }
        
        if (selectedCurrency === 'PHP') {
            console.log('\nPHP is the base currency. Rate is always 1.00');
            rl.question('\nPress Enter to continue...', function() {
                recordExchangeRate();
            });
            return;
        }
        
        rl.question('Exchange Rate (to PHP): ', function(rateStr) {
            const rate = parseFloat(rateStr);
            
            if (isNaN(rate) || rate <= 0) {
                console.log('Invalid rate!');
                rl.question('\nPress Enter to continue...', function() {
                    recordExchangeRate();
                });
                return;
            }
            
            app.rates[selectedCurrency] = rate;
            console.log('\nExchange rate for ' + selectedCurrency + ' set to ' + rate.toFixed(2) + ' PHP');
            
            rl.question('\nBack to Main Menu (Y/N)? ', function() {
                showMainMenu();
            });
        });
    });
}

function currencyExchange() {
    console.clear();
    console.log('=================================');
    console.log('   FOREIGN CURRENCY EXCHANGE');
    console.log('=================================\n');
    
    console.log('Source Currency Option:');
    console.log('[1] Philippine Peso (PHP)');
    console.log('[2] United States Dollar (USD)');
    console.log('[3] Japanese Yen (JPY)');
    console.log('[4] British Pound Sterling (GBP)');
    console.log('[5] Euro (EUR)');
    console.log('[6] Chinese Yuan Renminbi (CNY)\n');
    
    rl.question('Source Currency: ', function(fromChoice) {
        const currencies = ['PHP', 'USD', 'JPY', 'GBP', 'EUR', 'CNY'];
        const fromCurrency = currencies[parseInt(fromChoice) - 1];
        
        if (!fromCurrency) {
            console.log('Invalid currency!');
            rl.question('\nPress Enter to continue...', function() {
                currencyExchange();
            });
            return;
        }
        
        rl.question('Source Amount: ', function(amountStr) {
            const amount = parseFloat(amountStr);
            
            if (isNaN(amount) || amount <= 0) {
                console.log('Invalid amount!');
                rl.question('\nPress Enter to continue...', function() {
                    currencyExchange();
                });
                return;
            }
            
            console.log('\nExchange Currency Options:');
            console.log('[1] Philippine Peso (PHP)');
            console.log('[2] United States Dollar (USD)');
            console.log('[3] Japanese Yen (JPY)');
            console.log('[4] British Pound Sterling (GBP)');
            console.log('[5] Euro (EUR)');
            console.log('[6] Chinese Yuan Renminbi (CNY)\n');
            
            rl.question('Exchange Currency: ', function(toChoice) {
                const toCurrency = currencies[parseInt(toChoice) - 1];
                
                if (!toCurrency) {
                    console.log('Invalid currency!');
                    rl.question('\nPress Enter to continue...', function() {
                        currencyExchange();
                    });
                    return;
                }
                
                // Convert through PHP as base
                const phpAmount = amount * app.rates[fromCurrency];
                const result = phpAmount / app.rates[toCurrency];
                
                console.log('\nExchange Result:');
                console.log(amount.toFixed(2) + ' ' + fromCurrency + ' = ' + result.toFixed(2) + ' ' + toCurrency);
                
                rl.question('\nConvert another currency (Y/N)? ', function(another) {
                    if (another.toUpperCase() === 'Y') {
                        currencyExchange();
                    } else {
                        showMainMenu();
                    }
                });
            });
        });
    });
}

function showInterest() {
    console.clear();
    console.log('=================================');
    console.log('   SHOW INTEREST AMOUNT');
    console.log('=================================\n');
    
    if (!app.accountName) {
        console.log('Please register an account first!');
        rl.question('\nPress Enter to continue...', function() {
            showMainMenu();
        });
        return;
    }
    
    console.log('Account Name: ' + app.accountName);
    console.log('Current Balance: ' + app.balance.toFixed(2) + ' ' + app.currency);
    console.log('Interest Rate: 5% per annum\n');
    
    rl.question('Total Number of Days: ', function(daysStr) {
        const days = parseInt(daysStr);
        
        if (isNaN(days) || days <= 0) {
            console.log('Invalid number of days!');
            rl.question('\nPress Enter to continue...', function() {
                showInterest();
            });
            return;
        }
        
        console.log('\n' + '='.repeat(50));
        console.log('Day\t|\tInterest\t|\tBalance');
        console.log('='.repeat(50));
        
        const annualRate = 0.05;
        let balance = app.balance;
        
        for (let i = 1; i <= days; i++) {
            const dailyInterest = app.balance * (annualRate / 365);
            balance = balance + dailyInterest;
            console.log(i + '\t|\t' + dailyInterest.toFixed(2) + '\t\t|\t' + balance.toFixed(2));
        }
        
        console.log('='.repeat(50));
        
        rl.question('\nBack to Main Menu (Y/N)? ', function() {
            showMainMenu();
        });
    });
}

console.log('\nWelcome to Banking Application!\n');
showMainMenu();