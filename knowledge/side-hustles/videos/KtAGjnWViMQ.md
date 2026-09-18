# Now is the Best Time to Make Money with Content (AI Tutorial)
Channel: The Koerner Office | https://www.youtube.com/watch?v=KtAGjnWViMQ | Published: 2026-02-18

Step-by-step tutorial for building a free/cheap AI automation that monitors competitor/inspiration content creators so you never run out of content ideas for a niche content-creation side hustle.

- **Core idea/business model**: pick a content niche (business ideas, local events, parenting hacks, motorcycle reviews, power washing, etc.), post consistently, build an audience, then monetize. The claimed bottleneck isn't the niche or monetization -- it's the daily grind of finding fresh content ideas. Solution: automate discovery of what top creators in your niche are already posting (used as inspiration/repurposing material, not literal copying -- "your spin, your personality").
- **Automation built (~30 min setup, ~$5/month to run)**:
  - Host an n8n automation instance on **Hostinger VPS** (hostinger.com/kerneroff, discount code "kernerofficer" for extra 10% off on top of a listed 61% off deal; choose the n8n OS template).
  - Connect Google APIs (Gmail + Google Sheets) via **Google Cloud Console**: create a project, enable Gmail API and Sheets API, configure OAuth consent screen (external audience, add your own email(s) as test users), add OAuth scopes (Gmail read/send/modify; Sheets read/write).
  - In n8n: build a workflow triggered on a schedule (e.g. daily at midnight, adjustable -- more frequent runs cost more in credits).
  - Use an **RSS Read** node pointed at a YouTube channel's RSS feed. Get the feed URL for free by going to the channel page, "View Page Source," then Ctrl+F "RSS" to find the feed URL directly -- avoids giving email to third-party RSS-URL generator sites.
  - Feed RSS output into a **Google Sheets** node ("append or update row") to log publish date, channel name, and video title into a tracking spreadsheet (requires creating Google OAuth credentials in Cloud Console and connecting via the redirect URL).
  - Duplicate the RSS Read node per additional channel you want to track (just swap in each channel's RSS URL) -- easy to scale to 10, 20+ channels.
  - Add an **Aggregate** node (aggregate individual fields -- titles, links) before sending email, so you get one consolidated digest email instead of one email per video (his first pass without this sent a flood of individual emails).
  - Add a **Gmail "send message"** node (HTML email type, using an expression/HTML template) to email yourself a weekly/daily digest of what tracked channels have posted.
- **Cost comparison**: ~$5/month all-in (Hostinger VPS; RSS feeds, Sheets, and n8n itself are free) vs. $30-$300/month for comparable commercial content-research tools.
- **Extension ideas mentioned**: sell access to your own version of this tool; build a "spy on your competitor" service and sell a $30/month subscription that emails other creators/business owners everything their named competitor channel posts each morning; browse the n8n template library for other pre-built automations, or build and sell your own template.

Caveats: it's Google-account-based (OAuth setup is the fussiest part -- has to be redone once per Google project, not per channel); scraping can throw occasional errors on some entries ("nothing we can't fix"); more frequent automation runs increase Hostinger/credit costs; positions this as removing the "finding ideas" bottleneck but explicitly says consistency in actually creating/posting content is still the hard part left to the user.
