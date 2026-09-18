# Watch Me Build an AI Agency in 24 Minutes
Channel: The Koerner Office | https://www.youtube.com/watch?v=r5yL8PW8Kyc | Published: 2025-11-13

- Idea: AI consulting/implementation agency -- build custom AI voice agents (using GoHighLevel) for local home-service businesses (this episode targets roofing contractors) and sell as a monthly service. Pitch: the agent answers calls, quotes jobs, books inspections 24/7 so the business never misses a call.
- Why roofing: roofers are relatively tech-savvy, already used to paying agencies hundreds-to-thousands/month for marketing/lead tools, and sell a high-ticket product (bigger willingness to pay than the previous 56-minute-challenge target, garage door repair).
- Lead sourcing (Outscraper):
  - Search Google Business Profile subcategories, not just the obvious category. "Roofing service" is a rarer/mistaken subcategory than "roofing contractor" -- less-targeted-by-competitors, lower competition (est. ~200 businesses/state mistakenly listed there).
  - Filter geographically to cities near you (for credibility/local trust) -- example used Dallas, Fort Worth, Plano, Frisco zip codes.
  - Require phone number in advanced parameters.
  - Cost: scraping ~950-978 leads cost between $1.50 and $10.
- Lead list cleanup process (Google Sheets): remove duplicate phone numbers; filter to mobile-only numbers (exclude toll-free/landline/unknown/VOIP); strip "LLC" from business names for a cleaner look; fill blank city fields with a fallback (e.g. "DFW"); randomize order with `=RAND()` + sort before trimming to a manageable batch size (to avoid a biased sample skewed toward one city); example trimmed 431 valid unique leads down to 100 to text.
- Voice agent build (GoHighLevel):
  - Create AI voice agent from scratch; pick a voice (tested several ElevenLabs-style voices, chose one named "Hope" for a conversational tone); model used: GPT-4o at "two cents per minute."
  - Set safety parameters: max call time, idle time, reminder frequency, response speed (fast), LLM temperature (leave at 0) -- this guards against runaway costs (example given: an 8-hour stuck call costing $100).
  - Knowledge base: ask ChatGPT to draft the FAQ knowledge base for the target industry (e.g. "create a knowledge base based on everything you know about the roofing business... most questions revolve around getting a quote and setting up an inspection... make the business name/location easily swappable"). If your first prompt isn't good enough, ask ChatGPT to instead return the "perfect prompt" to get what you want, then use that. Knowledge base supports up to 25,000 characters; use custom bracketed variables (e.g. business name, location) so it can be swapped per customer.
  - Knowledge base cost note: more characters = more tokens = more per-call cost passed through to the client, so keep it as lean as possible while still functional.
  - Strategy: build a generic/placeholder version of the agent (e.g. "Greg's Roofing") rather than sending real prospects a link/number tied to a specific company -- only personalize and hand over the live number after a prospect says yes, to make the personalized version feel more impressive ("mind-blowing").
  - Use a phone number local to the target market's area code for trust.
- Outreach (texting via "Straight Text" app -- syncs iMessage across iPhone/MacBook, sends personalized bulk texts from a CSV):
  - Personalize each text with the business's own name/city pulled from the CSV (via mail-merge-style fields) -- improves response rate and avoids spam/carrier flagging from identical mass messages.
  - Message tested: "Good morning. Do you still own [business]? -Chris" then a separate follow-up pitch message once they reply.
  - Sending 100 texts took ~7-8 minutes (roughly one every 5 seconds).
- Results from texting 100 roofing leads: 4 failed to deliver (bad numbers), 96 delivered, 15 responses (~15-16% response rate after ~1 hour, expected to roughly double by end), 3 of those 15 were qualified "warm leads" (20% of responders, i.e. ~3% of total list).
- Follow-up/closing tactics: for non-responders, had the AI voice agent itself proactively call the prospect's own number, then followed up referencing "that call you got -- I built that" as a hook; for people who asked "who is this," gave name/company (Repeat Leads, repeatleads.com) and offered a live demo call from a local-area-code number; explicitly offered the agent free (e.g. "free for 10 calls a month" or free until proven) to remove friction and get first users, planning to monetize a more custom/expanded version later.
- Math/ROI framing: at $1,000/month per client, texting ~1,000 businesses should theoretically net enough leads/closes to reach $10,000/month; scaling to 10,000 texts targets $100,000/month, assuming consistent ~3-4% lead conversion from cold text outreach.
- Recommended approach for viewers: don't have to pick roofing specifically -- there are 1,000+ local/home-service verticals; test 3-5 industries in parallel, track response rates/sentiment, and double down on whichever performs best.

Caveats/risks:
- Response rate was low for texting (15% respond, only ~3% become warm leads) -- host attributes some of this to over-identifying himself as a marketer rather than sounding like a genuine potential customer; suggests a shorter, less "salesy" first message (e.g. "Hey, do you inspect roofs? -Chris") would work better, and that having the prospect's actual first name (available via Outscraper with more setup time) would raise response rates further.
- Higher-ticket/tech-savvy verticals like roofing are also more competitive -- many other marketers are already targeting them; whether the additional competition is worth it depends on how saturated that specific local market already is, and you won't know until you test it.
- One prospect responded angry to being texted -- inherent risk of cold outreach.
- Some tactics shown (using the AI agent to proactively call leads who never agreed to be called) are explicitly flagged by the host as "a little risky" / ethically gray ("is it devious? Is it genius? Yes, to both, maybe").
- Video includes an affiliate link/sponsorship for GoHighLevel and promotes the host's own paid community (PlaymakersAI) for AI consultants -- treat tool endorsements accordingly.
