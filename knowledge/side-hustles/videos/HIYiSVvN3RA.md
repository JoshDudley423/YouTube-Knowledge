# I Built an App the Government Doesn't Want You to See
Channel: The Koerner Office | https://www.youtube.com/watch?v=HIYiSVvN3RA | Published: 2026-04-07

Chris Koerner solo episode/demo (sponsored by Replit — includes referral link/credit offer). He builds and publishes a real product live in the episode: GovDealFinder (govdealfinder.com), a simplified search tool for US federal government contracts, using Replit Agent 4 with no coding experience.

## The business idea
- Problem: SAM.gov, the official US federal government contract-listing site, gets ~2.2 million visits/month but has notoriously poor UX/filtering ("looks like it was built in 2004") — many people give up before finding relevant, biddable contracts.
- Market size cited: the US government spent $834 billion on contracts in the prior year; ~$200 billion of that is specifically set aside for small businesses, including businesses with zero employees (solo operators qualify).
- Product concept: a clean, simple front-end that lets a user type in what their business does and select their state, then surfaces matching government contracts they can actually bid on, with dollar amounts shown, plus a "quiz" feature to help users self-check eligibility for a given contract, and email alerts for new matching contracts.
- Named GovDealFinder (name auto-suggested by the Replit tool itself); domain purchased for $12.
- Note: Replit is a sponsor of the episode/podcast — this is presented as a real workflow the host uses "every day off camera," not purely a paid placement, but the commercial relationship should be kept in mind.

## Steps/process — how the app was built (the main content of the video)
1. Draft a short plain-English description of what you want (Chris used ~3 sentences: "I want to build a website that piggybacks on SAM.gov and makes it easier to use").
2. Feed that description to Claude and ask it to turn it into a detailed, well-structured build prompt suitable for a coding agent ("make me a beautiful prompt to give to Replit").
3. Paste that Claude-generated prompt into Replit Agent 4 and tell it to build the app.
4. Get a SAM.gov API key (free — anyone can register an account at sam.gov, then Account Details > API > Generate API Key) and paste it into Replit so the app can pull live contract data directly via the official API rather than scraping.
5. Replit Agent 4's key differentiator versus older AI coding tools: it runs multiple agents in parallel on different parts of the app simultaneously (e.g., one agent builds the backend/data-pulling layer, another builds the frontend search UI, another generates the landing page copy/design) instead of requiring a slow one-step-at-a-time describe/wait/review/fix loop.
6. Initial full working app (functional frontend + backend + landing page) was generated in about 4 minutes from the prompt.
7. Iterate with additional natural-language prompts to add features: e.g., "add an email subscription so people get notified when new contracts match their filters, show a pop-up after their first search prompting sign-up." Also asked it to generate 3-5 SEO-oriented blog posts to help the site rank in search for "government contracts."
8. Use Replit's built-in "skills" (accessible via a "+" button) for auxiliary tasks like a branding generator or SEO optimizer/auditor.
9. Use Replit's "Infinite Canvas" feature to generate and compare multiple alternative versions/redesigns of a page (e.g., asked for "surprise me" and got three distinct landing-page hero-section concepts to choose from) without overwriting the current version — fully reversible exploration.
10. Generate a fundraising pitch deck directly from the built app's context: prompted "make a five-slide pitch deck explaining what this tool does, the market size, and how it helps small businesses find set-aside opportunities" — produced a six-slide deck automatically, since the tool already had full context on the product.
11. Replit supports team collaboration (multiple people editing/reviewing the same project) if you want a human to refine the AI's output.
12. Final step: register a matching domain name (checked availability, bought for $12) and connect/publish it directly from Replit with one click.

## Concrete numbers
- SAM.gov traffic: ~2.2 million visits/month.
- Total US government contract spend (prior year): $834 billion.
- Small-business set-aside portion: ~$200 billion/year, open to businesses with under 10 employees, including solo/zero-employee operators.
- Cost/time comparison: a comparable custom-built website would normally cost an estimated $10,000-$20,000 and require a development team plus weeks/months of iteration — versus ~4 minutes for Replit Agent 4 to generate the initial working app from a detailed prompt (additional iteration time not separately quantified).
- Domain purchase: $12.
- Replit referral: sign-up via the host's link gives $10 in free credits.

## Tools/platforms named
- Replit (specifically "Agent 4," the multi-agent AI app-building feature) — core build tool
- Claude — used first, specifically to convert a rough plain-English idea into a detailed, well-engineered build prompt for Replit
- SAM.gov — the official federal contract database and its free public API (the actual data source the app is built on top of)
- Google Trends, Ahrefs — mentioned as the tools used beforehand to validate that "government contracts" was a topic with real search/opportunity signal, prompting the choice of this niche (not demoed in detail in this episode)

## Caveats / risks
- This episode is sponsored by Replit, and the host explicitly discloses this — worth weighing when assessing how representative the "4 minutes to build" framing is of a typical user's experience versus an experienced power user's.
- The generated "quiz" and other UI copy came from the AI-generated prompt without the host fully reviewing/understanding every line ("I don't know what half this stuff means, but it looks smart, so I'm going with it") — a caution that AI-drafted product copy/features may need closer human review before being treated as production-ready, especially anything making claims to end users.
- The host explicitly declined one AI-generated landing-page variant that included a fabricated "success story" testimonial ("I don't want to lie") — a specific reminder to review AI-generated marketing copy for false/unverifiable claims before publishing.
- The tool as built is a thin, though genuinely more usable, layer over publicly available SAM.gov data — the core value proposition is UX/search simplification, not proprietary data or exclusive access; monetization strategy (e.g., subscription, lead-gen, ads) is not detailed in this episode beyond building the free-to-use tool and collecting emails.
