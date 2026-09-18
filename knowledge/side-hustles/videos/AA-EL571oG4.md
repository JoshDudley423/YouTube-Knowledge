# I Built an AI Agent That Works While I Sleep (Cheap & Easy)
Channel: The Koerner Office | https://www.youtube.com/watch?v=AA-EL571oG4 | Published: 2025-11-28

Tutorial-style video: build and sell "AI agent" automations for content creators/small businesses using self-hosted n8n on a cheap VPS. Walks through building one concrete example (a "content multiplier" agent that turns a YouTube video into LinkedIn posts) and pitches it as a productized service/agency side hustle.

## The idea / business model
- Sell AI automation "agents" as a service to creators and small business owners (real estate, fitness, restaurants, healthcare, podcasters, etc.) who are overwhelmed by social media/content distribution or repetitive digital tasks.
- Pricing model proposed: charge $300–$800/month per client for an ongoing agent (e.g., 10 clients ≈ $5,000/month, largely profit since the underlying tooling is cheap), or a $2,000 one-time setup fee per client (10 clients ≈ $20,000 in setup revenue).
- Suggested growth path: build templates for yourself first, build a few for friends for free as case studies/portfolio, then start charging. After 5–10 clients, hire a virtual assistant to help fulfill. Projected timeline: 6–12 months to scale to 10–30 clients at $10,000–$30,000/month combined (setup + ongoing fees).
- Framed as low-competition because most small business owners/creators aren't yet using these AI tools, so a service provider can charge less than competitors while delivering equal or better results.

## Concept: automation vs. AI agent
- Automation (e.g., Zapier/Make) = fixed "if this then that" logic; doesn't understand context or adapt.
- AI agent = uses an LLM (OpenAI, Grok, Claude, Perplexity, etc.) plus memory and tool/API connections to make decisions and act with less explicit rule-writing (e.g., can read an inbox, classify emails as leads/spam/personal, add CRM entries, and schedule follow-ups without a written rule for each case).

## Tools and setup (concrete, step-by-step)
- **n8n**: open-source, AI-native workflow/automation builder (more powerful than Zapier/Make but with a steeper learning curve); self-hosting it (rather than using n8n's own cloud hosting) avoids per-automation cost scaling.
- **Hostinger**: VPS (virtual private server) provider used to self-host n8n cheaply. Recommended plan: KVM-2 (has "enough juice to run multiple AI agents"). Sponsor discount code "KOERNEROFFICE" for extra 10% off Black Friday/Cyber Monday pricing (VPS as low as $5.99/month during that sale — note this is a sponsored plug, treat baseline pricing with that in mind).
- **Cost warning ("cloud pricing trap")**: n8n's own cloud hosting starts at $20/month but can balloon past $100/month once you run several automations or one very active one; self-hosting on a VPS is the same flat cost regardless of running 10 or 100 agents.
- n8n comes with hundreds/thousands of pre-built community templates (browsable in-app) covering things like auto-generating TikTok videos with AI avatars, automating YouTube thumbnail creation and social publishing, etc. — a resource for finding sellable automation ideas without building from scratch.

## Concrete build walkthrough: "content multiplier" agent (YouTube → LinkedIn posts)
1. Trigger: scheduled to run daily (e.g., 6:00 a.m.).
2. Node: "Get many videos" via connected YouTube credentials/channel ID, sorted by date descending, limited to 1 result — pulls the most recent video's ID.
3. Node: HTTP POST request to youtube-transcript.io (third-party transcript-fetching API; requires its own account/API key sent as an Authorization header) to retrieve the video's transcript, passing the video ID in the JSON body.
4. Transcript arrives split into chunks — combined into one text block before the next step.
5. Node: OpenAI ("Message model") call using GPT-5, with a system prompt framing it as a "helpful, intelligent YouTube transcript analyst," and a user prompt instructing it to extract the 3–5 most valuable insights from the transcript for LinkedIn (actionable business advice, stories, quotable moments).
6. Three parallel OpenAI nodes (duplicated from one template) each write a LinkedIn post at a different length from those extracted insights: short (under 300 characters, strong hook under 120 characters on two lines, no emojis, "spartan tone"), medium (800–1,200 characters), and long (1,200–1,500 characters).
7. Result: three ready-to-post LinkedIn drafts generated automatically per new video, fully reusable for any client's YouTube channel/podcast, not just the demo's own.
- Practical build note: each downstream node must be wired to pull from the correct upstream node's output — a common setup mistake (shown live: fixing a node that wasn't connected to the transcript-combination step).
- To make output quality higher for a real client, add example/reference posts (the video's demo skips this for simplicity, but flags it as the natural next improvement).

## Caveats
- Hostinger/VPS segment is a paid sponsorship (discount code given) — evaluate independently rather than treating as neutral advice.
- The specific revenue projections ($5K–$30K/month) are aspirational framing from the host, not verified case studies of anyone actually running this.
- Relies on a third-party transcript API (youtube-transcript.io) and OpenAI API access/costs, which are ongoing dependencies/costs not fully broken out in the video.
