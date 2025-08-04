# OxidizedBB

## The oxidized BBS. SSG everything you can, and SSR all the things!

### Technical docs (for dev, while still in early development)

we use a two-pass model for building the page we serve. 

the first pass is done at compile-time; we ask for a config file to compile with, and we use that config file, alongside
askama, to produce a bunch of templates for layout themes. this produces a bunch of html files in `_cache`, with the necessary placeholders for the second pass

the second pass, the tool we use here is tbd. still working on the layout subsystem. but when we get a request, the second pass is SQL-aware and fully dynamic, so it can fill in all the
placeholders left by the first pass. this is the final HTML file served to the user (and then immediately discarded by the server, obviously).

so the SSR pipeline ends up looking like:

#### compile-time:
        1. user compiles oxidizedbb with a custom bbconfig.toml
                - askama spits out a bunch of theme files in _cache
        2. user starts the server
#### request-time
        1. end-user visits the site
        2. oxidizedbb hears a request for "index"
        3. oxidizedbb checks to see if user is logged in (cookie). if they are, it hits the db and queries their theme. otherwise, it uses the default theme
        4. oxidizedbb grabs the matching theme's header, category listing, and footer
        5. oxidizedbb fills in the header (current user, last visited), category listing (categories and their forums and stats), and footer (info of user choice) with info from sql db
        6. oxidizedbb passes header back to apachegit  
