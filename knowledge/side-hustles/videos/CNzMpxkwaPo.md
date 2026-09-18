# How To Start a $10K/Month AI Automation Agency (No Code)
Channel: The Koerner Office | https://www.youtube.com/watch?v=CNzMpxkwaPo | Published: 2025-06-17

Interview with Flo Crivello, founder of Lindy (a no-code AI agent platform). Focus is on what AI agents can concretely do today and how to build a business around setting them up for others.

## AI agent vs. AI automation — the distinction
- AI automation: each step is isolated ("an island") — context has to be manually passed between steps; good for simple, linear, "once-and-done" tasks (e.g., new lead comes in → research → qualify → email).
- AI agent: powered by an LLM throughout, retains memory/context across the whole interaction (short and long-term), can handle conversations and ambiguity, decides what to do next itself. Use an agent whenever there's back-and-forth conversation with a human, or whenever you're not 100% sure what should happen next.
- Agents are described as forgiving/robust compared to traditional automation or code — a messy or badly-worded prompt with many steps can still work because the agent "limps along" and figures it out, whereas one small error in code/traditional automation breaks the whole thing.
- Current state of agents characterized as "AI interns," not yet full "AI employees" — good for well-defined sequences of tasks (even non-identical ones, i.e. a "playbook"), but not yet good at truly open-ended, ambiguous, strategic work (e.g., "figure out our marketing strategy" — the kind of judgment call a VP-level hire would need to make). Expected to close that gap "in a couple of years."

## The three biggest categories of real-world Lindy use cases (~80% of usage)
1. **Sales**: lead generation, enrichment, qualification, personalized outreach at scale, AI voice/phone agents for follow-ups, scheduling meetings, sending confirmation emails/texts, joining and taking notes on the call, sending post-meeting follow-ups — all while retaining full cross-channel context (web chat, SMS, phone).
2. **Customer support**: full automation across channels (SMS, email, Intercom, Zendesk, Freshdesk, WhatsApp, phone).
3. **Personal assistant tasks**: email triage/drafting, meeting scheduling and prep briefings, meeting recording/note-taking, CRM maintenance (weekly auto-suggestions of new contacts to add based on calendar activity), and a "daily digest" agent that compiles LinkedIn info + email history + meeting notes into one pre-meeting briefing per person you're meeting that day.

## Concrete "less common but powerful" (the "20%") use cases described
- **Internal transparency/reporting agent**: an agent sitting at the boundary between company and customers (reads every support ticket, sales call, customer email) and posts a daily Slack summary to the whole company — described as making "black box" companies transparent; executives can then interactively ask follow-up questions about flagged issues directly in Slack, and the agent can be corrected/trained in natural language ("don't flag this kind of thing again") and it remembers going forward.
- **Deep research agent**: give it a person's name; it searches Google for interviews/podcast appearances, transcribes and summarizes each one in parallel ("agent swarm"), then produces a synthesized "summary of summaries" — including finding cross-interview overlap/non-overlap and recurring themes/quotes. Framed as ideal for podcast guest prep or founder/company research, especially since podcast content is otherwise not very searchable.
- **Recruiting agent**: given a target role and a list of target companies, it searches for matching candidates (e.g., "director of SMB sales" at specific companies), checks your inbox to avoid duplicate outreach, drafts and sends personalized cold outreach emails, and sends automated follow-ups days later — demoed finding candidates and sending first outreach within ~5 minutes of the initial request.
- **Niche lead-gen example**: a music-studio client (makes music tracks for video games) set up an agent to scrape the Steam store for independent game studios below a certain sales threshold (i.e., too small to have in-house music staff), then find and message their contact info automatically.
- **Nuanced human-like judgment example**: an agent handling a meeting-reschedule request responded appropriately to an emotional situation (someone's child was hospitalized) by expressing sympathy and deliberately not immediately pushing to reschedule — waiting several days before checking back in — described as behavior the founder never explicitly programmed.

## Business model: building an "AI agency" on top of Lindy
- One cited real example (unverified, single anecdote relayed by Flo): a Lindy user reportedly generates ~$10,000/month running a Lindy-based agency — model is: post free educational content on LinkedIn/YouTube showing off a specific automation ("check out this cool Lindy, here's how I built it"), give away the template, and a portion of viewers ask to have it custom-built/set up for them instead of DIY-ing it.
- Pricing pattern described: a one-time setup fee plus ongoing monthly maintenance in the range of ~$500-$1,000/month per client.
- Math: even at $500/month per client, 10 clients = $5,000/month; framed as low-effort to sustain since much of the top-of-funnel comes from free content rather than paid ads or cold sales.
- Recommended niche selection strategy for launching an agency: target an industry you already have direct familiarity/audience in — "every industry can benefit from AI agents" since every industry has employees whose repetitive work can be automated/augmented.

## Voice-agent-specific guidance
- Voice/phone agents work especially well when the prospect/customer is not tech-savvy, is frequently on the go, or is too busy to take a Zoom call — examples given: plumbers/blue-collar field reps (each missed call = missed business), restaurants (answering repetitive "are you open / can I book a table" questions during rush hours), and companies with large call-center-style volume (e.g., ISPs/telecoms).
- Disclosure nuance: for transactional, low-relationship interactions (e.g., calling a restaurant to check hours), customers are generally fine with — or even prefer — talking to an AI without it needing to announce itself, since it beats waiting on hold. For higher-stakes/relationship-based interactions (e.g., B2B sales, SDR outreach), the guest suggests it's better to disclose that it's an AI upfront.
- Contrary to the interviewer's initial assumption, voice agents may work better for low-tech/blue-collar B2B (e.g., selling pipe fittings) than for tech-savvy enterprise SaaS buyers, because the target buyer in the blue-collar case is busy/on-the-go and doesn't want a Zoom call.
- A humans-still-needed caveat: fully AI-run sales (closing deals end-to-end) isn't there yet — humans build relationships better than AI (an "anti-AI bias" persists), so AI agents currently handle everything up to and around the call (research, scheduling, prep briefs, follow-up), while an actual human still closes.
- A UX detail worth noting: some automation platforms let you insert a deliberate delay before an automated reply so it doesn't feel suspiciously instant (an instant multi-paragraph reply to a sensitive email can feel off-putting/inauthentic).
- Feature idea mentioned: "ringless voicemail" (e.g., via a service like Slybroadcast) to drop a voice-cloned follow-up voicemail without the phone actually ringing — proposed as a way for a sales agent's cloned voice to leave post-call action-item voicemails.

## Practical building resources
- lindy.ai/templates — hundreds of pre-built templates (e.g., lead generator flows); users copy/customize rather than building from scratch. A recruiter-specific template was noted as a gap at time of recording.
- The underlying LLM matters and is swappable: Lindy defaults to Claude (Sonnet) for most workflows, explicitly recommends against GPT-4o for powering workflows (calling it "not good" despite being fine for casual chat) — guest personally favors Claude Sonnet models for both quality and "writing style" (harder to detect as AI-generated).
- Context window handling: modern context windows (cited up to ~1-2 million tokens) rarely hit limits; when they do, Lindy can auto-compress context or use Retrieval-Augmented Generation (RAG) against a large knowledge base so the live context window (a few thousand tokens) stays small even with a "trillion-token" knowledge corpus.
- Multi-business/multi-brand context routing: use branching logic nodes (e.g., "if this is Business A, use Knowledge Base A") so one inbox serving multiple businesses (e.g., a holding company's 8 different businesses) can be routed to the right knowledge/response logic.
- Named integrations for lead generation: **People Data Labs** (Lindy's lead database, described as better/less stale than **Apollo**), and **Apify** (a large scraper marketplace with "thousands" of pre-built scrapers) — both singled out as the most powerful integrations on the platform.

## Caveats / limitations raised
- Team/collaboration accounts on Lindy were not yet available at time of recording (in development).
- "Computer use" (agent controlling your actual computer/screen) was requested by users but not yet shipped (in development).
- Setting up your own custom agent still required some effort at time of recording; an "agent-builds-agents" copilot feature was in private/admin-only testing.
- The presenter (Chris) explicitly flags that most "AI SDR" sales-agent hype is still early/unproven at scale ("a party trick" for many funded startups), though Lindy's founder claims some customers do rely entirely on AI SDRs with no human SDRs (closing itself still requires a human).
- All agents are ultimately bounded by the underlying LLM's capabilities and context window, despite compression/RAG workarounds.

## Tools/platforms named
- Lindy — the no-code AI agent platform being discussed (interview subject's company); lindy.ai/templates for pre-built flows.
- 11 Labs, Synthflow — voice-agent/voice-cloning tools mentioned as integration options for outbound calling.
- Slybroadcast — ringless voicemail drop service mentioned as a possible integration idea.
- People Data Labs, Apollo — B2B lead databases (People Data Labs preferred by Lindy's founder).
- Apify — large scraper marketplace integrated with Lindy.
- Zapier — referenced as a comparison point for simpler automation vs. agent-based workflows.
