<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    onMount(() => {
        const slug: string | null = $page.params["slug"];
        if (slug != null) {
            fetch(`${location.origin}/api/get-url/${slug}`).then((res) => {
                if (res.ok) {
                    const reader = res.body?.getReader();
                    reader?.read().then((url) => {
                        const encoder = new TextEncoder();
                        const decoder = new TextDecoder();
                        const expanded_url = decoder.decode(
                            url?.value ??
                                Uint8Array.from(
                                    encoder.encode(location.origin),
                                ),
                        );
                        // Prevent the shortened link from being in the history
                        // and relocate the user
                        window.location.replace(expanded_url);
                    });
                } else {
                    console.error(res.statusText);
                }
            });
        }
    });
</script>
