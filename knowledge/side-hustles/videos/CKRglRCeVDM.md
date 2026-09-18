# How I Use Web Scraping to Find Products That Sell for 20x the Price
Channel: The Koerner Office | https://www.youtube.com/watch?v=CKRglRCeVDM | Published: 2026-03-10

Solo episode: step-by-step method for finding profitable Amazon FBA products to source from Alibaba/China and resell, using Amazon-scraping data plus AI analysis instead of guessing/picking a familiar category.

## The core idea/business
- Amazon FBA (fulfilled-by-Amazon) reselling of imported/China-sourced products remains viable despite the "Amazon FBA course" fad dying off years ago — Amazon itself is still huge and people are still making millions doing this.
- Core method: scrape Amazon best-seller data by category, use a specific ratio (ratings count ÷ best-seller rank) to spot "land grab" opportunities — products selling well but not yet dominated by an entrenched leader — then source a similar/matching product from Alibaba and list a competing version on Amazon.

## Step-by-step process
1. **Get category browse node IDs**: ask an AI (Claude/ChatGPT/Grok) for the 5-7 digit Amazon browse node/query code for target categories (host provides a free pre-built Google Sheet of these codes in the video description, no email required).
2. **Scrape Amazon best sellers** via Oxylabs' Web Scraper API (Amazon → Best Sellers scraper): input the browse node ID, set `parsing: true` for structured output, pick domain (.com for US) and locale/zip code (critical — wrong locale returns wrong country's results/currency).
3. Export results as JSON, convert to CSV (can hand the JSON to Claude and ask it to "convert this to CSV and make the Amazon URLs complete and clickable"), then paste into Google Sheets. Resulting columns of interest: ratings count, price, star rating, title, ASIN, and constructed product URL.
4. **Find the "review count vs. sales rank mismatch"** — described as "the single most valuable signal that most people miss": a product with a strong (high) best-seller rank but low review count (e.g., under ~300) signals unmet demand in a category nobody has locked down yet. High rank + high review count = saturated/established leader (avoid); high rank + low review count = still up for grabs.
   - Formula: add a column = (ratings count) ÷ (best-seller rank position), sort ascending; low scores flag under-reviewed-but-well-ranked products.
5. **Estimate real sales volume from review counts**: only roughly 1-5% (rule of thumb; used 2% in the walkthrough) of buyers leave a review, so multiply review count by ~50 (at 2%) to estimate total units sold historically.
6. **Look for statistical outliers** within a category's top sellers using spreadsheet conditional formatting (color scale) on the ratings column — products near the top of best-seller rank but with unusually low ratings relative to peers are the target.
7. **Use AI (Claude) to analyze *why* an outlier product is winning**: screenshot the top few competing listings and ask Claude to compare them and identify what's driving one's outperformance. Also run a second, blind/unbiased prompt ("which of these shows the best opportunity for me to compete directly, sourcing from Alibaba?") to cross-check your own hypothesis without leading the model.
   - Worked example (pet feeder category): a 2-in-1 gravity-fed cat food/water feeder outperformed competitors because it combined two functions, had zero moving parts/electronics (fewer defects/returns — Amazon's algorithm ranks lower-return-rate listings higher), and was priced at a value sweet spot ($29.99, delivering "two products" perceived value vs. single-function competitors at similar price).
8. **Source a matching product from Alibaba**: use Oxylabs' Alibaba scraper with a descriptive search term, or manually reverse-image-search the winning Amazon product photo on Alibaba to find near-identical listings.
   - Concrete example numbers: the cat feeder sold on Amazon for ~$30; matching Alibaba supplier price was $1.55/unit at 100-unit MOQ (order of $155 total for 100 units, before shipping).
   - Landed cost estimate: ~$4-5/unit sea freight + ~30% tariffs to ship to an Amazon warehouse, using $2/unit product cost (conservative) → ~$7/unit landed cost.
   - Amazon fees for a "large bulky" size-tier item: $10-15/unit (covers shipping to consumer, storage, etc.) → total per-unit cost $16-21, leaving ~$9/unit net profit at the same $30 sale price, a ~30% net margin — compared favorably to what FBA sellers used to net 5-15 years ago.
   - Tactic: shrinking the product slightly to drop into a cheaper Amazon size-fee tier (e.g., "large" instead of "large bulky") can save several dollars per unit in fees — cited example: dropping price by $2 (to $28) while cutting size-tier fees by $3 nets an extra $1/unit and may even boost conversion via lower price.
9. **Repeat across categories**, going from micro-niche to a wider category to find higher review-count "tangential" opportunities (e.g., instead of trying to compete directly with commodity paper towels/toilet paper, target adjacent accessory categories like paper towel holders where a similar high-search-volume audience exists but competition is thinner). Example: a $5 stainless-look paper towel holder selling well on Amazon sourced from Alibaba for $0.40-$1.77/unit.

## Additional scoring signals/strategies for picking a winning product
- **"One-star test"**: read negative/1-star reviews and count how many cite the *same* complaint (quality control, shipping damage, color issue, etc.) — if roughly 40%+ share the same complaint, that's a specific, fixable flaw you can correct in a competing product and win on lower return rate (which Amazon's algorithm rewards with better search placement).
- **Photo quality/branding signal**: if a competitor's Amazon listing uses the exact same stock photo as the Alibaba supplier listing, that signals lazy/low-effort branding — an opportunity to win by using real lifestyle photography (product in use, not just on a white background); the first image is what buyers actually look at (most don't read long titles).
- **Variation opportunity**: scrape a product's review/Q&A sections for repeated requests ("I wish this came in black/bigger/bundled") and take those requests back to the Alibaba manufacturer to source a variant that fills the gap.
- **Shipping weight/dimension fee-tier arbitrage**: check Amazon's fee thresholds by weight/size and see if shrinking a product slightly drops it into a cheaper fee tier — pure profit gained without changing the product's function.
- **Rating distribution shape**: a 4.0 average made of mostly 5-star + a few 1-star ratings suggests an isolated/fixable defect; a 4.0 average spread evenly across 1-5 stars suggests a deeper, harder-to-fix product problem.
- **Seasonal trend overlay via Google Trends**: check whether a product's demand is seasonal (seasonal businesses are fine — "that Christmas tree vendor... prints money... makes all of his money in 2 months") or has sustained upward search trend (e.g., cited "trace letters" search term trending steadily up over 5 years on Google Trends — treated as a strong non-seasonal buy signal).
- **Amazon's Choice badge ambiguity**: if multiple competing products in a category all carry an "Amazon's Choice" badge, that signals Amazon hasn't settled on a clear category leader — opportunity still open.
- Titles of top-performing listings tend to follow a "what it is, what it does, who it's for" structure (identified by asking Claude to find commonalities across top book titles in a niche); age-gating a product ("for ages 4-8") narrows the buyer but increases relevance/conversion. This title structure is portable to other marketplaces (e.g., Etsy printables).

## Categories specifically walked through
- Pet accessories (example: 2-in-1 gravity cat feeder).
- Household/health & wellness (example: paper towel holder).
- Books & education (host's personal favorite category — "very unsexy, very boring, very profitable"); referenced a separate video about buying pallets of books for $1 and reselling individually on Amazon/eBay for a profit. Concrete example: an "Oxford Reading Tree" style book set found for $33/33 books on Alibaba vs. $167 for a comparable product on Amazon; a kindergarten workbook sourced at $0.80 vs. selling for $7 with 8,700 reviews on Amazon.

## Caveats / risks raised
- Explicit trademark/copyright warning: China-based suppliers are not reliably careful about IP; host advises being cautious/ethical, and to prefer generic (non-branded, non-copyrighted) products like generic workbooks/coloring books over anything resembling a licensed property (e.g., "Oxford Reading Tree" branding).
- A newly-listed Alibaba supplier with unusually low pricing could mean either an unreliable/unproven supplier or an aggressive new entrant trying to win its first customers — you don't know which until you test.
- The review-count-to-sales-estimate math is a rough approximation (1-5% of buyers leave reviews) and shouldn't be taken as precise.
- Just because a category looks saturated (e.g., paper towels dominated by Bounty) doesn't mean there's no room — but conversely, don't assume you can simply out-compete category giants; the play is adjacent/tangential niches, not head-on competition with entrenched national brands.
- The presenter explicitly does not go on to actually build/launch the specific products analyzed in the episode — framed as an educational walkthrough, not a live case study with real revenue results.

## Tools/platforms named
- Oxylabs — the scraping tool used for both Amazon best-seller data and Alibaba product/supplier search (sponsor of the episode; offers free trial up to 2,000 scraped results, discount code "CHRIS" for 20% off).
- Claude / ChatGPT / Grok — used to fetch category browse node IDs, convert JSON to CSV, analyze screenshots of competing listings, and identify title-structure patterns.
- Google Sheets — where scraped data is organized, filtered, and conditionally formatted.
- Google Trends — used for seasonal/demand-trend validation of product search terms.
- Alibaba — sourcing platform for matching supplier products (used both direct search and reverse image search).
