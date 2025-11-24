const fs = require('fs');
const readline = require('readline');

let projects = [];

// UTILITY FUNCTIONS
function ask(question) {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });
    return new Promise(resolve => {
        rl.question(question, answer => {
            rl.close();
            resolve(answer);
        });
    });
}

function parseCSV(line) {
    const result = [];
    let current = '';
    let inQuotes = false;
    
    for (let char of line) {
        if (char === '"') {
            inQuotes = !inQuotes;
        } else if (char === ',' && !inQuotes) {
            result.push(current.trim());
            current = '';
        } else {
            current += char;
        }
    }
    result.push(current.trim());
    return result;
}

function toNumber(str) {
    return parseFloat(str.replace(/,/g, ''));
}

function daysBetween(start, end) {
    return Math.floor((new Date(end) - new Date(start)) / 86400000);
}

// DATA LOADING AND CLEANING
function loadData() {
    const data = fs.readFileSync('dpwh_flood_control_projects.csv', 'utf-8');
    const lines = data.split('\n');
    const headers = parseCSV(lines[0]);
    const rawData = [];
    
    for (let i = 1; i < lines.length; i++) {
        if (!lines[i].trim()) continue;
        const values = parseCSV(lines[i]);
        if (values.length < headers.length - 5) continue;
        
        const row = {};
        headers.forEach((h, j) => row[h] = values[j] || '');
        rawData.push(row);
    }
    
    console.log(`Processing dataset... (${rawData.length} rows loaded, `, '');
    
    // Clean and filter data
    projects = rawData.filter(p => {
        const year = parseInt(p.FundingYear);
        if (year < 2021 || year > 2023) return false;
        
        const approved = toNumber(p.ApprovedBudgetForContract);
        const contract = toNumber(p.ContractCost);
        const delay = daysBetween(p.StartDate, p.ActualCompletionDate);
        
        if (isNaN(approved) || isNaN(contract) || isNaN(delay)) return false;
        
        p.ApprovedBudget = approved;
        p.ContractCostNum = contract;
        p.CostSavings = approved - contract;
        p.CompletionDelayDays = delay;
        
        return true;
    });
    
    console.log(`${projects.length} filtered for 2021-2023)`);
    console.log(' Data loaded successfully!\n');
}


// REPORT GENERATION
function groupBy(array, keyFn) {
    return array.reduce((acc, item) => {
        const key = keyFn(item);
        if (!acc[key]) acc[key] = [];
        acc[key].push(item);
        return acc;
    }, {});
}

function median(arr) {
    const sorted = [...arr].sort((a, b) => a - b);
    const mid = Math.floor(sorted.length / 2);
    return sorted.length % 2 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

function avg(arr) {
    return arr.reduce((s, v) => s + v, 0) / arr.length;
}

function sum(arr) {
    return arr.reduce((s, v) => s + v, 0);
}

function report1() {
    const groups = groupBy(projects, p => `${p.Region}|${p.MainIsland}`);
    
    const results = Object.entries(groups).map(([key, projs]) => {
        const [region, island] = key.split('|');
        const totalBudget = sum(projs.map(p => p.ApprovedBudget));
        const medianSavings = median(projs.map(p => p.CostSavings));
        const avgDelay = avg(projs.map(p => p.CompletionDelayDays));
        const highDelayPct = (projs.filter(p => p.CompletionDelayDays > 30).length / projs.length) * 100;
        
        let effScore = avgDelay > 0 ? (medianSavings / avgDelay) * 100 : 0;
        effScore = Math.max(0, Math.min(100, effScore));
        
        return {
            Region: region,
            MainIsland: island,
            TotalBudget: totalBudget.toFixed(2),
            MedianSavings: medianSavings.toFixed(2),
            AvgDelay: avgDelay.toFixed(2),
            HighDelayPct: highDelayPct.toFixed(2),
            EfficiencyScore: effScore.toFixed(2)
        };
    });
    
    results.sort((a, b) => parseFloat(b.EfficiencyScore) - parseFloat(a.EfficiencyScore));
    writeCSV('report1_regional_summary.csv', results);
    return results;
}

function report2() {
    const groups = groupBy(projects, p => p.ContractorName);
    
    const results = Object.entries(groups)
        .filter(([_, projs]) => projs.length >= 5)
        .map(([name, projs]) => {
            const totalCost = sum(projs.map(p => p.ContractCostNum));
            const avgDelay = avg(projs.map(p => p.CompletionDelayDays));
            const totalSavings = sum(projs.map(p => p.CostSavings));
            
            let reliability = (1 - (avgDelay / 90)) * (totalSavings / totalCost) * 100;
            reliability = Math.max(0, Math.min(100, reliability));
            
            return {
                Rank: 0,
                Contractor: name,
                TotalCost: totalCost.toFixed(2),
                NumProjects: projs.length,
                AvgDelay: avgDelay.toFixed(2),
                TotalSavings: totalSavings.toFixed(2),
                ReliabilityIndex: reliability.toFixed(2),
                RiskFlag: reliability < 50 ? 'High Risk' : 'Low Risk'
            };
        });
    
    results.sort((a, b) => parseFloat(b.TotalCost) - parseFloat(a.TotalCost));
    results.slice(0, 15).forEach((r, i) => r.Rank = i + 1);
    
    const top15 = results.slice(0, 15);
    writeCSV('report2_contractor_ranking.csv', top15);
    return top15;
}

function report3() {
    const groups = groupBy(projects, p => `${p.FundingYear}|${p.TypeOfWork}`);
    const baseline = {};
    
    const results = Object.entries(groups).map(([key, projs]) => {
        const [year, type] = key.split('|');
        const avgSavings = avg(projs.map(p => p.CostSavings));
        const overrunRate = (projs.filter(p => p.CostSavings < 0).length / projs.length) * 100;
        
        if (year === '2021') baseline[type] = avgSavings;
        
        const yoyChange = baseline[type] && year !== '2021' 
            ? ((avgSavings - baseline[type]) / baseline[type]) * 100 
            : 0;
        
        return {
            FundingYear: year,
            TypeOfWork: type,
            TotalProjects: projs.length,
            AvgSavings: avgSavings.toFixed(2),
            OverrunRate: overrunRate.toFixed(2),
            YoYChange: yoyChange.toFixed(2)
        };
    });
    
    results.sort((a, b) => {
        if (a.FundingYear !== b.FundingYear) return parseInt(a.FundingYear) - parseInt(b.FundingYear);
        return parseFloat(b.AvgSavings) - parseFloat(a.AvgSavings);
    });
    
    writeCSV('report3_annual_trends.csv', results);
    return results;
}

function generateSummary() {
    const contractors = new Set(projects.map(p => p.ContractorName));
    const summary = {
        total_projects: projects.length,
        total_contractors: contractors.size,
        global_avg_delay: parseFloat(avg(projects.map(p => p.CompletionDelayDays)).toFixed(2)),
        total_savings: Math.round(sum(projects.map(p => p.CostSavings)))
    };
    
    fs.writeFileSync('summary.json', JSON.stringify(summary, null, 2));
    return summary;
}

// OUTPUT FORMATTING
function writeCSV(filename, data) {
    if (!data.length) return;
    const headers = Object.keys(data[0]);
    const csv = [
        headers.join(','),
        ...data.map(row => headers.map(h => row[h]).join(','))
    ].join('\n');
    fs.writeFileSync(filename, csv);
}

function displayPreview(num, data, title) {
    console.log(`\nReport ${num}: ${title}`);
    console.log('\n' + title);
    console.log('(Filtered: 2021-2023 Projects)\n');
    
    if (!data.length) {
        console.log('No data to display.\n');
        return;
    }
    
    const headers = Object.keys(data[0]);
    const w = 18;
    
    console.log('| ' + headers.map(h => h.padEnd(w)).join(' | ') + ' |');
    console.log('|' + headers.map(() => '-'.repeat(w + 2)).join('|') + '|');
    
    data.slice(0, 2).forEach(row => {
        console.log('| ' + headers.map(h => String(row[h]).padEnd(w)).join(' | ') + ' |');
    });
    
    console.log(`\n(Full table exported to report${num}_*.csv)\n`);
}

// MAIN PROGRAM
async function main() {
    console.clear();
    
    while (true) {
        console.log('   FLOOD CONTROL DATA ANALYSIS PIPELINE');
        console.log('\nSelect Language Implementation:');
        console.log('[1] Load the file');
        console.log('[2] Generate Reports');
        console.log('[3] Exit\n');
        
        const choice = await ask('Enter choice: ');
        
        if (choice === '1') {
            try {
                loadData();
            } catch (error) {
                console.log('\n Error: ' + error.message);
                console.log('Make sure dpwh_flood_control_projects.csv is in the same folder.\n');
            }
        } else if (choice === '2') {
            if (!projects.length) {
                console.log('\n Error, Please load the file first (option 1).\n');
                continue;
            }
            
            console.log('\nGenerating reports...');
            console.log('Outputs saved to individual files…\n');
            
            const r1 = report1();
            const r2 = report2();
            const r3 = report3();
            const summary = generateSummary();
            
            displayPreview(1, r1, 'Regional Flood Mitigation Efficiency Summary');
            displayPreview(2, r2, 'Top Contractors Performance Ranking (Top 15, >=5 Projects)');
            displayPreview(3, r3, 'Annual Project Type Cost Overrun Trends');
            
            console.log('Summary Stats (summary.json):');
            console.log(JSON.stringify(summary, null, 2));
            
            console.log('\nBack to Report Selection (Y/N):');
            await ask('');
        } else if (choice === '3') {
            console.log('\n Thank you for using the Flood Control Data Analysis Pipeline!\n');
            process.exit(0);
        } else {
            console.log('\n Invalid choice. Please enter 1, 2, or 3.\n');
        }
    }
}

main();