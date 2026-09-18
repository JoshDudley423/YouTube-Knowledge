# I Started an Online Business in 56 Minutes (Beginner Friendly)
Channel: The Koerner Office | https://www.youtube.com/watch?v=5mClVr2cd5U | Published: 2025-08-01

## The idea
- AI after-hours answering agent agency for local home service businesses. Most small-business owners (garage door repair, plumbing, HVAC, etc.) don't answer calls after hours and lose high-ticket jobs to voicemail; you sell them an AI voice agent that answers 24/7, sounds human, collects name/address, and can transfer urgent callers straight to the owner.
- Picked garage doors specifically because: it's high-ticket ($400-$2,500 per repair), generates genuine after-hours emergencies, and (unlike plumbing/HVAC/electrical) hasn't already been consolidated by private equity.
- Framed as broadly repeatable across "thousands of home service niches": foundation repair, sprinkler repair, landscaping, roofing, gutter cleaning, etc.

## Numbers
- Total time from idea to a business owner saying "yes, interested": 56 minutes, on camera, unstaged.
- Outscraper lead scrape: 1,000 max results requested across Fort Worth/McKinney/Allen (DFW suburbs); returned ~513-542 results for ~$1.11 (separately quoted an estimate of $9.28 for a larger pull).
- After data cleanup (dedup, cell-phone-only filter), left with 153 valid cell numbers from the original list.
- Sent cold texts to 50 business owners: ~50% response rate within the video; ~16/50 (32%) responded within first 10 minutes; extrapolated to expect 60-70% eventual response rate.
- 1 explicit "yes, interested" out of 50 texted = 2% yes rate (not just response rate).
- Revenue model floated: charge $20-$200 per lead (per after-hours call that converts), OR a flat monthly retainer (~$500/month per business).
- Back-of-envelope market sizing: estimated ~20,000-50,000 garage door repair companies in the US; if you signed 200 of them at $500/month, that's $100K/month ($1.2M/year), with near-zero costs and no employees needed.
- Total out-of-pocket cost for the whole test: "under $100" (scrape ~$1-9, texting ~50 cents for 50 texts, HighLevel free trial).

## Process/steps
1. Pick a niche: high-ticket + likely to generate after-hours emergency calls + not yet dominated by private equity roll-ups.
2. Scrape leads with Outscraper (Google Maps data scraper): search the niche keyword + several target cities, get phone numbers, export to Google Sheets.
3. Clean the data in Sheets: delete unneeded columns, keep business name/address/phone; standardize phone formats (strip "+1" and dashes); use conditional formatting to flag rows containing "repair" in the name; remove duplicate phone numbers; filter to cell-phone-only numbers (reduces list size a lot but improves text deliverability).
4. Build the AI voice agent in HighLevel's "AI Employee" / Voice AI feature: create custom agent, name the fictitious business, pick a voice, set to inbound-only, write a short welcome message, optionally build a knowledge base (can be generated via ChatGPT prompt: background info, instructions for handling queries, script), set phone number pool (use a local area code number) and hours (24/7). Test by calling it yourself.
5. Bulk text the cleaned lead list using StraightText (sends from Mac/iPhone/Android/PC): first message just validates they own the business ("Do you still fix garage doors? Found you on Google Maps") — short (<160 chars to avoid splitting into 2 SMS / avoid RCS fallback), includes first name/city for personalization, sent via iMessage (blue bubble) for higher trust/open rate since ~2/3 of US uses iPhone.
6. Once they confirm, send the pitch: e.g. "I created an AI voice agent for garage door repair companies that answers 24/7 — you only pay $50 when you close a lead from one of those calls. No contract, no money upfront, free setup."
7. Only build the AI agent for real prospects after getting interest — don't over-build before validating demand.
8. Deliberately used a no-cost/no-contract/no-upfront offer to remove friction while validating; plans to add contracts/monthly retainers/upfront pricing later once traction is proven.

## Tools/platforms named
- Outscraper.com (Google Maps lead scraper, affiliate link available for discount)
- HighLevel / GoHighLevel (AI Voice Agent / "AI Employee" feature, 30-day free trial via affiliate link vs standard 14-day)
- StraightText (bulk SMS/iMessage broadcast tool)
- ChatGPT (to generate the AI agent's knowledge base/script)
- Google Sheets (data cleaning)

## Caveats / risks
- No LLC, no website, no logo, no business plan needed to validate — deliberately skipped until there's a paying customer.
- A "yes, interested" is not a closed deal — most deals never close; this is a demand-validation exercise, not proof of revenue.
- Data quality is messy: Google Maps has no dedicated category for some niches (e.g. "garage door repair"), so results include irrelevant listings (suppliers vs. repair companies) that must be manually filtered; only an estimated 85-90% of scraped "garage door supplier" listings actually do repair work.
- Bulk/undifferentiated texts can trigger carrier filtering ("undifferentiated messages" warning) — using personalization (first name, city) reduces this risk.
- Pricing given away for free/no-risk now is a placeholder; expect to add contracts, upfront fees, or higher retainers once the model is proven — don't let price be the reason a first test fails.
