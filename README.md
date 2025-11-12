# Sympatica: API and Frontend for the Empatetica E4 Wristband
Since the original application and cloud solution for this product was sunset in early 2025, I figured I'd port it to a open-source application.

This (work-in-progress, barely complete) project is a web-based UI for the E4 built using Vue and Django. Made partly to use in my undergraduate final-year project, partly for my ECS639U Web Programming module, and mostly to spite Empatica and open source this bit of gear.

I humbly ask for your patience while I get this to a runnable state. Here are some notes if you really want to get it working on your end:
- Run Django in dev mode, run npm in dev mode
- Only Chrome will work since I'm relying on the Web Bluetooth API. Blame Firefox.
- If you're using Linux you'll have to go into `chrome://flags` and manually enable Web Bluetooth as it's a experimental feature.
- The SQLite database has some weird stuff in it, don't question it.
- Writing the E4 readings to the DB doesn't work yet.

Other than those minor issues, enjoy. You can contact me [here](mailto:e.t.unal@se23.qmul.ac.uk) if you so desire.
