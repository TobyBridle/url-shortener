<script lang="ts">
  let url: string | null;
</script>
<form
  method="POST"
  action="/api/generate-url"
>
  <h1>URL Shortener</h1>
  <h2>Paste the URL to be shortened!</h2>
  <input
    type="url"
    name="url"
    aria-label="URL to Shorten"
    bind:value={url}
    on:emptied={(url = null)}
  />
  <button type="submit" disabled={url == null}>Shorten</button>
</form>

<style>
  form input {
    color: black;
  }
  form button {
    outline: 0;
    border: 0;
    border-radius: 0.3em;
    background-color: white;
  }

  :global(body) {
    font-family:
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      Roboto,
      Oxygen,
      Ubuntu,
      Cantarell,
      "Open Sans",
      "Helvetica Neue",
      sans-serif;
    color: white;
    background-color: rgba(14, 14, 14, 1);
  }
  :global(body::before) {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    /* background-color: black; */
    background-size: 40px 40px;
    background-image: radial-gradient(
      circle,
      rgba(255, 255, 255, 0.2) 1px,
      rgba(255, 255, 255, 0) 1px
    );
  }
</style>
