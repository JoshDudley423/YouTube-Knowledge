# The Easiest Way to Actually Make Money With AI
Channel: The Koerner Office | https://www.youtube.com/watch?v=03DjE7j0Suw | Published: 2026-04-17

Interview with Corey Ganim (Return My Time / Build With AI podcast) on running an "AI assessment/audit" agency for small businesses — diagnosing where a business can use AI, then upselling implementation work.

## The core business idea

- Sell an "AI assessment" (deliberately not called an "audit" — testing showed "assessment" is less intimidating/less resisted) to small business owners: interview them about their business, then deliver a report identifying where AI/software tools could save them time or money.
- Origin story: a friend (a mortgage broker) offered to pay $1,000 just to have Corey shadow him for a day pointing out AI opportunities. Not scalable to physically shadow people, so he productized it as a call-based assessment instead. Talked to 50-100 business owners since; found ~99 out of 100 need this kind of service.

## Process evolution (three iterations)

1. **V1 — screen recording**: had clients install Loom and record 60-120 minutes of a normal deep-work session for review. Abandoned — too much friction/unfamiliarity for clients, and made them self-conscious about being watched (e.g. "do I need to pause if I go on Reddit?").
2. **V2 — live 45-minute Zoom call**: interview the business owner directly, take the transcript, feed it to Claude, research tools, deliver a report + do a follow-up call. Bottleneck: limited number of Zoom calls one person can run per day.
3. **V3 (current, as of the episode) — AI voice agent**: business owner calls a phone number 24/7 and talks for 20-30 minutes to a voice agent ("Annie," built on **Retell.ai**) that asks a bank of questions (general questions first, narrowing into industry-specific ones — question bank built up across ~8-9 industries so far). The call transcript is piped automatically to a separate AI agent that generates the report. Removes the live-call bottleneck entirely.
   - Anyone starting out doesn't need this complex agent infrastructure: simplest version is literally pasting the raw conversation transcript into Claude and prompting it to find off-the-shelf tools that address the pain points mentioned (see exact prompt below).

## Report structure (the deliverable)

Built as a Gamma (AI doc/slideshow tool) presentation, generated from a Word doc (.docx) the AI agent outputs. Free assessment-report template available at **audittemplate.ai**. Sections:
1. Executive summary — restates the client's pain points and quantifies the outcome (e.g., "8 hours/week reclaimed" from implementing 4-5 recommended tools).
2. Effort-vs-impact matrix — maps every pain point mentioned on the call, prioritizing "quick wins" (low effort, high impact).
3. Recommended solutions — the meat of the report; specific off-the-shelf tools mapped to specific pain points. Examples cited: **Fathom.ai** (free meeting-notes/transcription copilot) recommended because "97% of people still aren't using these tools"; **Dash This** (non-AI SaaS dashboard tool, $42/month) recommended to a wedding-venue operations manager who was spending 2 hours every Saturday manually copying Google Analytics/Meta Ads/Google Ads data into a spreadsheet then a PowerPoint — saved her ~8 hours/month for $42/month.
4. A "four-day quick win plan" — breaks implementation into daily bite-size steps (e.g., "Day 1: set up Fathom and connect your calendar") so clients aren't overwhelmed.
5. Upsell-opportunities section — bigger, heavier-lift projects (e.g., setting up a full CRM) planted as "what comes after quick wins."
6. Financial-impact quantification — converts time saved into dollars (using a conservative $100/hr assumption even though many owners' real hourly value is higher) minus the monthly cost of recommended tools, to make ROI explicit. Advice from Chris Koerner: put this ROI slide at both the top AND the end of the report as a hook.
7. A booking link for a 30-minute follow-up call, delivered ~48 hours after the initial interview; report is sent in advance of the follow-up call, which is then a screen-share walkthrough.

## Pricing / revenue numbers

- First 2-3 assessments done for free to get testimonials and iron out the process.
- Price ladder as demand validated: free → $200 → $500 → $1,000 (current price at time of recording).
- Advice: don't underprice early — pricing too low ($200) makes clients undervalue the recommendations and makes a later jump to a $3,500 upsell feel too large; charging $1,000 up front makes a $3,500 upsell feel proportionate and clients take the assessment more seriously (sunk-cost effect).
- Upsell examples and pricing:
  - **Process optimization** (fixing an inefficient process before automating it, e.g. cutting a 15-step process to 7 steps): $3,000-4,000 per engagement.
  - **Automating a fixed process** (after optimization) via Zapier/Make.com: separate engagement, another $3,000-10,000.
  - **Simple automation builds**: 1-3K is "pretty standard"; concrete example — a $1,500 Zapier automation for a wedding-venue client that auto-formatted new-client Asana projects, saving the ops manager ~30 minutes per new client that she'd previously set up manually.
  - **Full CRM setup** (e.g., GoHighLevel) for a business not using one: "easily a $3,000-5,000 proposal."
  - **Custom knowledge system / custom GPT**: example — a business brokerage owner doing 5-7 listings/year was getting 200-400 inbound emails per listing in week one, ~95% of which were the same ~10 questions; built a custom GPT trained on the listing's marketing package (the "CIM") to auto-answer buyer questions, cutting inbound to ~5%. Reusable system prompt means it can be resold for every future listing at no extra build cost.
  - **Speed-to-lead AI agent** (Corey's very first paid AI engagement): built in Make.com (works same in Zapier) — when a lead submits a web form, the agent looks the person up, sends a personalized (not generic) response, and converses to book a call/tour. Example: a wedding venue whose in-person salesperson couldn't respond to web leads for 6-8 hours; after the agent went live, tour bookings increased immediately. Quantified value: avg deal size $11,000, 40% tour-to-close rate, so even 1-2 extra tours/month = meaningfully more revenue — framed as the easiest upsell to justify because it ties directly to revenue rather than vague time savings.

## Exact Claude prompt pattern used for turning a transcript into recommendations

"I've attached the transcript of a conversation I had with a local business owner. Your job is to go out on the internet and find any AI tools or software tools that they can implement that are off-the-shelf that can fix the pain points you've identified from the transcript." Tool-directory sites named for sourcing recommendations: **Futurepedia** and **There's An AI For That**.

## Which clients/industries work best

- Worked across many industries in practice: realtors, business brokerages, wedding venue operators, a barber school.
- Rule of thumb given: best fit is a business with a team (not a solo operator) — ideally 5-50 employees — where the owner has "hands in a lot of different pots"; for those, the assessment is a "no-brainer" and will typically surface 3-5 upsell opportunities.

## Customer acquisition — "7 ways to sell AI services with no existing following"

1. **Host a free local "AI for small business" meetup** (via Luma or Meetup.com). Give a genuine value-only presentation (no pitch), which positions you as the de facto local expert. Example: partnered with the realtor who sold Corey his house to host it inside her office; converted attendees into assessment clients.
2. **Door-knock local businesses** specifically to talk about AI (framed as much easier than door-knocking to sell insurance/home services because of current AI hype/curiosity) — lead with value, let it become a sales conversation only if it naturally goes there.
3. **Do free assessments for warm contacts** (e.g., a local CEO peer group of ~11-12 business owners) to get testimonials, then tactfully upsell those same relationships afterward (e.g., free audit → $1,500 paid automation build).
4. **Host free recurring "office hours"** at a co-working space or a local business's office (e.g., a realtor branch) where you just show up and answer AI questions — low/no downside even in the worst case, since you build a contact network regardless.
5. (Implied from the "7 ways" article referenced but not fully enumerated in the transcript — only 4 of the 7 are detailed on-air.)

## Key caveats / advice

- You do not need to be an AI expert — just "one step ahead of the average client," achievable with roughly a week of self-study.
- Specialize in one primary upsell early rather than trying to sell every possible upsell — spreading across too many offerings dilutes focus; pick a "bread and butter" upsell to productize (e.g., always upselling a custom GPT).
- Prioritize upsells with a clearly quantifiable financial impact (revenue or hard cost savings) over vague "this will make things easier" pitches — these close far more easily.
