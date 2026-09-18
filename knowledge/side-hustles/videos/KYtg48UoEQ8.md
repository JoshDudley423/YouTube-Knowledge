# I Asked AI to Help Me Beat the Stock Market
Channel: The Koerner Office | https://www.youtube.com/watch?v=KYtg48UoEQ8 | Published: 2025-10-15

Not a side-hustle-income video -- a tutorial on using AI/vibe-coding tools to build a personal stock-screening/portfolio-tracking app. Relevant mainly as an example of "vibe coding" a tool with AI + a stock-picking framework.

- **Core investment thesis tested**: companies that are (a) still led by their founder and (b) benefit from network effects (value increases as more users join, e.g. eBay, Facebook, Tesla Supercharger network, StubHub) outperform the S&P 500.
- **Tools/process used to build the app**:
  - Used ChatGPT "deep research" to find all S&P 500 companies that are both founder-led and have network effects -- returned 26 companies.
  - Built a spreadsheet modeling $100 invested in each of the 26 companies on their IPO date vs. $100 in the S&P 500 on the same date, tracked to today.
  - Used **Replit** (specifically their "Agent 3" feature, which auto-tests/breaks/fixes the app in a loop) to vibe-code a portfolio tracker app that visualizes this, benchmarked against the S&P 500, with 1-day/1-week/1-month/1-year/5-year/10-year/20-year views.
  - Expanded the dataset to the full S&P 500 in a Google Sheet (name, ticker, IPO price, shares needed for $100, current price via live Google Finance formula, market cap, PE ratio, founding year, S&P inclusion date, exchange, founder-led flag, network-effects flag).
  - Used **GPT for Sheets** (an AI-in-Google-Sheets tool) to auto-score every company 1-10 on "network effects strength" and "founder-led strength" -- cost him about $15 in credits to run across the whole list.
  - Fed the live Google Sheet (as an editable link) into Replit as the app's live data source, with filters for network-effect score, founder score, market cap, PE, and time horizon.
- **Result claimed**: over a 10-year default horizon, filtering to 8-10 on both network effects and founder-led returned 6 matching stocks (5 with valid IPO price data); portfolio return was 22.4% vs. S&P 500's 9% weighted / 12% unweighted return over the same period -- "over twice" the weighted return.
- Total build time: ~45 minutes of prompting.
- Final tool named "Neil Stocks" (NEL = Network Effects, Founder-Led); he registered nelstocks.com as a free tool gated behind an email signup (no cost to use).
- **AI prompts given as reusable "find stock ideas" templates** (for use in ChatGPT or similar):
  1. Small/micro caps with a moat, high ROIC, but low analyst coverage.
  2. Companies where insiders bought materially over the last 6 months but the stock is still down >20%.
  3. Stocks with high free-cash-flow yield but PE below peer median.
  4. Businesses benefiting from structural/secular tailwinds (regulation, demographic shift, tariffs) the market is ignoring.
  5. Disrupted industries: weak-balance-sheet incumbents vs. strong-balance-sheet disruptors (valuation mismatch), e.g. Nvidia vs. AMD.
  6. High short interest but improving fundamentals.
  7. Recurring revenue / customer stickiness / operating leverage that the market isn't crediting yet.
  8. Underfollowed stocks with improving margins and a growth inflection, or trading below private-market/liquidation value (his example: Coinbase ~3 years prior had more cash on its balance sheet than its market cap -- he went in heavily and says it's his best-performing holding).
  9. High barriers to entry / scale advantages in currently "boring" sectors (his example: a former Bitcoin-mining data-center company that pivoted to AI).

Caveats: this is a personal thesis/backtest, not verified investment advice; some IPO-date data was hard to find/incomplete (had to supplement Wikipedia with other AI tools); acknowledges he "took a risk" going in hard on Coinbase specifically because crypto was still seen as risky by many investors.
