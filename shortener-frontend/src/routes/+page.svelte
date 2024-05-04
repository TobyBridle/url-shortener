<script lang="ts">
    import ShortenTag from "../components/shorten_form.svelte";
</script>

<h1>Shorten Your URL</h1>
<ShortenTag />
<p>
Shortened URLs keep track of how many “hits” (uses of the shortened url) there are.
<br/>
This data is accessible to signed in users, yet is stored within the database regardless.
<br/>
<br/>
To create an account, <a href="#">click here</a>.
</p>
<footer><span>Made with ❤️</span><span>Source code is available on <a href="https://github.com/tobybridle/url-shortener">GitHub</a></span></footer>
<style>
    :root {
        --body-background: #f3f3f3;
        --subtext-foreground: #A4A4A4;
    }
    :global(body) {
        background-color: var(--body-background);
        padding: 5% 5%;
    }
    h1 {
        width: 20rem;
        font-size: 3.5rem;
        font-family: "Inter", sans-serif;
        font-weight: 900;
        text-transform: uppercase;
        padding-bottom: 2.5rem;
    }

    p, span {
        font-family: "Inter", sans-serif;
        color: var(--subtext-foreground);
        line-height: 1.4;
    }

    footer {
        position: absolute;
        bottom: 5%;

        display: flex;
        flex-direction: column;
    }
    
    @media only screen and (max-width: 500px) {
        h1 {
            font-size: 3.2rem;
            padding-bottom: 0;
        }
    }
</style>
