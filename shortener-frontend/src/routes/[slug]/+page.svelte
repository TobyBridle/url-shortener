<script lang="ts">
    import { page } from "$app/stores";
    import { redirect } from "@sveltejs/kit";
    let slug: string | null;
    $: slug = $page.params["slug"];
    let err: string | undefined;
    if (slug != null) {
        fetch(`/api/get-url/${slug}`).then(res => {
            if (res.ok) {
                const reader = res.body?.getReader()
                reader?.read().then(url => {
                    alert(url.value?.toString())
                    redirect(302, url.value?.toString() ?? "/")
                })
            } else {
                err = res.statusText;
            }
        })
    } else {
        console.log($page)
    }
</script>
{err}