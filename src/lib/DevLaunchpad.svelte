<script lang="ts">
    import { onMount } from "svelte";

    // ----- Props -----
    export let data: Record<string, string[]> = {
        Rust: [
            "https://github.com/prongbang/upstream",
            "https://crates.io/crates/cmdwrap",
            "https://crates.io/crates/lazychacha",
            "https://github.com/prongbang/watchx",
            "https://github.com/prongbang/postman-runner",
            "https://github.com/prongbang/fqrs",
            "https://github.com/prongbang/server",
            "https://github.com/prongbang/icongen",
            "https://github.com/prongbang/flutter-runner",
        ],
        GO: [
            "https://github.com/prongbang/callx",
            "https://github.com/prongbang/fiberhandler",
            "https://github.com/prongbang/excelmetadata",
            "https://github.com/prongbang/excelrecreator",
            "https://github.com/prongbang/fiberresp",
            "https://github.com/prongbang/localizegen",
            "https://github.com/prongbang/casbinrest",
            "https://github.com/prongbang/echoerror",
            "https://github.com/prongbang/goerror",
            "https://github.com/prongbang/fibererror",
            "https://github.com/prongbang/fiber-casbinrest",
            "https://github.com/prongbang/goredoc",
            "https://github.com/prongbang/gojwe",
            "https://github.com/prongbang/lazyxchacha",
            "https://github.com/prongbang/lazynacl",
            "https://github.com/prongbang/csvx",
            "https://github.com/prongbang/gocron",
            "https://github.com/prongbang/excelx",
            "https://github.com/prongbang/analyticsgen",
            "https://github.com/prongbang/phuslog_logger",
            "https://github.com/prongbang/lazychacha",
            "https://github.com/prongbang/lazyxsalsa",
            "https://github.com/prongbang/gojwt",
            "https://github.com/prongbang/goecdsa",
            "https://github.com/prongbang/wiremock",
            "https://github.com/prongbang/lazybob",
            "https://github.com/prongbang/staticfy",
            "https://github.com/prongbang/goenv",
        ],
        Flutter: [
            "https://github.com/prongbang/lazysecret",
            "https://github.com/prongbang/screen_protector",
            "https://github.com/prongbang/local_auth_crypto",
            "https://github.com/prongbang/aes_ctr",
            "https://github.com/prongbang/flutter_kiosk_mode",
            "https://github.com/prongbang/imgcompress",
            "https://github.com/prongbang/flutter_resizer_image",
            "https://github.com/prongbang/cached_memory_image",
            "https://github.com/prongbang/flutter_certificate_pinning",
            "https://github.com/prongbang/pin_keyboard",
            "https://github.com/prongbang/pin_dot",
            "https://github.com/prongbang/flutter_page_transition_plus",
            "https://github.com/prongbang/snapshot_widget",
        ],
        JavaScript: [
            "https://github.com/prongbang/lazyxchacha-js",
            "https://github.com/prongbang/lazyaesgcm-js",
        ],
        Swift: [
            "https://github.com/prongbang/SignatureBiometricSwift",
            "https://github.com/prongbang/ScreenProtectorKit",
            "https://github.com/prongbang/SecureStorageSwift",
            "https://github.com/prongbang/SecureBiometricSwift",
        ],
        Kotlin: [
            "https://github.com/prongbang/local_auth_signature",
            "https://github.com/prongbang/android-sheet-localization",
            "https://github.com/prongbang/sheet-localization-annotation",
            "https://github.com/prongbang/sheet-localization-processor",
            "https://github.com/prongbang/BottomSheetDialog",
            "https://github.com/prongbang/android-secure-storage",
            "https://github.com/prongbang/android-secure-biometric",
            "https://github.com/prongbang/android-biometric-signature",
            "https://github.com/prongbang/android-fqrs",
            "https://github.com/prongbang/thirdparty_keyboard_checker",
            "https://github.com/prongbang/localization",
            "https://github.com/prongbang/screen-protector",
            "https://github.com/prongbang/kiosk-mode",
            "https://github.com/prongbang/custom-layout-manager",
            "https://github.com/prongbang/mvi-flow",
            "https://github.com/prongbang/mvi-rx",
            "https://github.com/prongbang/snapx-layout",
            "https://github.com/prongbang/RijndaelCryptographyKt",
            "https://github.com/prongbang/SmartDialog",
            "https://github.com/prongbang/ScannerView",
            "https://github.com/prongbang/ErrorView",
        ],
        Program: [
            "https://github.com/prongbang/ai-upscaler",
            "https://github.com/prongbang/gifphy",
        ],
    };

    export let descriptions: Record<string, string> = {
        Rust: "Rust crates and CLI/Lib tools.",
        GO: "Libraries and utilities for Go, Fiber, Echo, and security.",
        Flutter:
            "Plugins and widgets focused on security, performance, and UX.",
        JavaScript: "Crypto libs (JS/TS) and small utilities.",
        Swift: "Secure storage / biometric / screen protection kits for iOS.",
        Kotlin: "Android components, MVI, secure storage/biometric, and UI utilities.",
        Program: "Standalone apps/scripts (e.g., AI and media).",
    };

    // ----- State -----
    let query = "";
    let activeFilter: string = "all";
    export let collapsed = new Set<string>();
    const toggleCollapsed = (category: string) => {
        const newSet = new Set(collapsed);
        if (newSet.has(category)) {
            newSet.delete(category);
        } else {
            newSet.add(category);
        }
        collapsed = newSet;
    };

    // ----- Helpers -----
    const dedupe = (arr: string[]) => {
        const seen = new Set<string>();
        return arr.filter((u) => (seen.has(u) ? false : (seen.add(u), true)));
    };

    $: cleanedData = Object.fromEntries(
        Object.entries(data).map(([k, list]) => [k, dedupe(list)]),
    );

    const makeTitle = (url: string) => {
        try {
            const u = new URL(url);
            if (u.hostname.includes("crates.io")) {
                const seg = u.pathname.split("/").filter(Boolean).pop();
                return seg ?? url;
            }
            const parts = u.pathname.split("/").filter(Boolean);
            if (u.hostname.includes("github.com") && parts.length >= 2)
                return parts[1];
            return u.hostname;
        } catch {
            return url;
        }
    };

    const hostType = (url: string) => {
        try {
            const h = new URL(url).hostname;
            if (h.includes("github.com"))
                return { host: "github.com", type: "GitHub" };
            if (h.includes("crates.io"))
                return { host: "crates.io", type: "Crate" };
            return { host: h, type: "Link" };
        } catch {
            return { host: "", type: "Link" };
        }
    };

    const matchQuery = (category: string, url: string) => {
        const q = query.trim().toLowerCase();
        if (!q) return true;
        const title = makeTitle(url).toLowerCase();
        return (
            title.includes(q) ||
            url.toLowerCase().includes(q) ||
            category.toLowerCase().includes(q)
        );
    };

    const openAll = (urls: string[]) => {
        urls.forEach((u) => window.open(u, "_blank"));
    };

    const copy = async (text: string) => {
        try {
            await navigator.clipboard.writeText(text);
        } catch {}
    };

    const setTheme = (mode: "dark" | "light") => {
        const root = document.documentElement;
        if (mode === "dark") {
            root.classList.add("dark");
        } else {
            root.classList.remove("dark");
        }
        localStorage.setItem("theme", mode);
    };

    const getSystemPrefersDark = () =>
        typeof window !== "undefined" &&
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: dark)").matches;

    const applyInitialTheme = () => {
        const saved =
            typeof window !== "undefined"
                ? localStorage.getItem("theme")
                : null;

        if (saved === "dark") {
            setTheme("dark");
        } else if (saved === "light") {
            setTheme("light");
        } else {
            setTheme(getSystemPrefersDark() ? "dark" : "light");
        }
    };

    const toggleTheme = () => {
        const isDark = document.documentElement.classList.contains("dark");
        setTheme(isDark ? "light" : "dark");
    };

    onMount(() => {
        applyInitialTheme();

        const mq = window.matchMedia("(prefers-color-scheme: dark)");
        const handler = (e: MediaQueryListEvent) => {
            const saved = localStorage.getItem("theme");
            if (!saved) setTheme(e.matches ? "dark" : "light");
        };
        mq.addEventListener?.("change", handler);

        const onStorage = (e: StorageEvent) => {
            if (e.key === "theme" && e.newValue) {
                setTheme(e.newValue as "dark" | "light");
            }
        };
        window.addEventListener("storage", onStorage);

        return () => {
            mq.removeEventListener?.("change", handler);
            window.removeEventListener("storage", onStorage);
        };
    });
</script>

<div>
    <!-- Header -->
    <header class="gradient-hero relative overflow-hidden">
        <div class="absolute inset-0 pointer-events-none opacity-[0.07]">
            <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg"
                ><defs
                    ><pattern
                        id="grid"
                        width="40"
                        height="40"
                        patternUnits="userSpaceOnUse"
                        ><path
                            d="M 40 0 L 0 0 0 40"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="0.5"
                        /></pattern
                    ></defs
                ><rect width="100%" height="100%" fill="url(#grid)" /></svg
            >
        </div>
        <div class="mx-auto max-w-7xl px-6 py-14 sm:py-16 lg:py-20">
            <div class="flex items-center justify-between gap-4">
                <div>
                    <h1
                        class="text-3xl sm:text-4xl lg:text-5xl font-extrabold tracking-tight"
                    >
                        Dev Launchpad
                    </h1>
                    <p
                        class="mt-3 max-w-2xl text-base sm:text-lg text-zinc-600 dark:text-zinc-300"
                    >
                        A curated hub of my coding projects, tools, and
                        libraries by language and platform — search, filter, and
                        open everything in a category with one click.
                    </p>
                </div>
                <div class="flex items-center gap-2">
                    {#if false}
                        <button
                            on:click={toggleTheme}
                            title="Toggle theme"
                            class="cursor-pointer inline-flex items-center gap-2 rounded-2xl border border-zinc-300/60 dark:border-zinc-700/80 px-3 py-2 text-sm font-medium bg-white/70 dark:bg-zinc-900/50 glass shadow-sm hover:shadow transition"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                class="h-4 w-4"
                                ><path
                                    fill="currentColor"
                                    d="M21.64 13a1 1 0 0 0-1.05-.14A8 8 0 1 1 11.14 3.41 1 1 0 0 0 13 2.36 10 10 0 1 0 22 14a1 1 0 0 0-.36-1z"
                                /></svg
                            >
                            <span>Theme</span>
                        </button>
                    {/if}
                </div>
            </div>

            <div class="mt-6 grid gap-3 sm:grid-cols-3">
                <label class="col-span-2">
                    <span class="sr-only">Search</span>
                    <input
                        bind:value={query}
                        type="search"
                        placeholder="Search projects or categories… (e.g. go, flutter, crypto)"
                        class="w-full rounded-2xl border border-zinc-300/70 dark:border-zinc-700/80 bg-white/80 dark:bg-zinc-900/60 glass px-4 py-3 text-sm outline-none focus:ring-2 focus:ring-indigo-500/60"
                    />
                </label>
                <div class="flex items-center gap-2 overflow-x-auto">
                    <button
                        on:click={() => (activeFilter = "all")}
                        class={`filter-chip active px-3 py-2 text-xs sm:text-sm rounded-full border border-zinc-300/70 dark:border-zinc-700/80 bg-white/70 dark:bg-zinc-900/60 glass ${
                            activeFilter === "all"
                                ? "ring-1 ring-indigo-500/50"
                                : ""
                        }`}
                    >
                        All
                    </button>
                    {#each Object.keys(cleanedData) as cat}
                        <button
                            on:click={() => (activeFilter = cat)}
                            class={`filter-chip px-3 py-2 text-xs sm:text-sm rounded-full border border-zinc-300/70 dark:border-zinc-700/80 bg-white/70 dark:bg-zinc-900/60 glass ${
                                activeFilter === cat
                                    ? "ring-1 ring-indigo-500/50"
                                    : ""
                            }`}
                        >
                            {cat}
                        </button>
                    {/each}
                </div>
            </div>
        </div>
    </header>

    <!-- Main -->
    <main class="mx-auto max-w-7xl px-6 py-10">
        <div class="space-y-10">
            {#each Object.entries(cleanedData) as [category, links]}
                {#if activeFilter === "all" || activeFilter.toLowerCase() === category.toLowerCase()}
                    {#key category}
                        {#if links.filter( (u) => matchQuery(category, u), ).length > 0}
                            <section class="group">
                                <div
                                    class="mb-4 flex flex-wrap items-end justify-between gap-3"
                                >
                                    <div>
                                        <h2
                                            class="text-2xl font-bold tracking-tight"
                                        >
                                            {category}
                                            <span
                                                class="align-middle ml-2 text-xs font-semibold px-2 py-1 rounded-full border border-zinc-300/70 dark:border-zinc-700/80 text-zinc-700 dark:text-zinc-200"
                                            >
                                                {links.filter((u) =>
                                                    matchQuery(category, u),
                                                ).length} links
                                            </span>
                                        </h2>
                                        <p
                                            class="text-sm text-zinc-600 dark:text-zinc-400 mt-1"
                                        >
                                            {descriptions[category] ?? ""}
                                        </p>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <button
                                            class="inline-flex items-center gap-2 rounded-xl border border-zinc-300/70 dark:border-zinc-700/80 px-3 py-2 text-sm bg-white/70 dark:bg-zinc-900/60 glass hover:shadow-sm card-hover"
                                            on:click={() =>
                                                openAll(
                                                    links.filter((u) =>
                                                        matchQuery(category, u),
                                                    ),
                                                )}
                                        >
                                            <svg
                                                class="h-4 w-4"
                                                xmlns="http://www.w3.org/2000/svg"
                                                viewBox="0 0 24 24"
                                                ><path
                                                    fill="currentColor"
                                                    d="M14 3v2h3.59L7 15.59 8.41 17 19 6.41V10h2V3z"
                                                /></svg
                                            >
                                            Open All
                                        </button>
                                        <button
                                            class="inline-flex items-center gap-2 rounded-xl border border-zinc-300/70 dark:border-zinc-700/80 px-3 py-2 text-sm bg-white/70 dark:bg-zinc-900/60 glass hover:shadow-sm card-hover"
                                            on:click={() =>
                                                toggleCollapsed(category)}
                                        >
                                            <svg
                                                class="h-4 w-4 transition"
                                                xmlns="http://www.w3.org/2000/svg"
                                                viewBox="0 0 24 24"
                                                style="transform: {collapsed.has(
                                                    category,
                                                )
                                                    ? 'rotate(-90deg)'
                                                    : 'rotate(0)'}"
                                                ><path
                                                    fill="currentColor"
                                                    d="M7 10l5 5 5-5z"
                                                /></svg
                                            >
                                            {collapsed.has(category)
                                                ? "Expand"
                                                : "Collapse"}
                                        </button>
                                    </div>
                                </div>

                                {#if !collapsed.has(category)}
                                    <div
                                        class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
                                    >
                                        {#each links.filter( (u) => matchQuery(category, u), ) as url (url)}
                                            <a
                                                href={url}
                                                target="_blank"
                                                rel="noopener"
                                                class="card-hover block rounded-2xl border border-zinc-200/70 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/60 glass p-4 focus:outline-none focus:ring-2 focus:ring-indigo-500/50"
                                            >
                                                <div
                                                    class="flex items-start justify-between gap-3"
                                                >
                                                    <div class="min-w-0">
                                                        <div
                                                            class="flex items-center gap-2 mb-1"
                                                        >
                                                            <span
                                                                class="inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-semibold border border-zinc-300/70 dark:border-zinc-700/80 text-zinc-700 dark:text-zinc-300 bg-zinc-50/60 dark:bg-zinc-800/60"
                                                            >
                                                                {hostType(url)
                                                                    .type}
                                                            </span>
                                                            <span
                                                                class="hidden sm:inline text-[10px] text-zinc-500 dark:text-zinc-400 truncate max-w-[120px]"
                                                            >
                                                                {hostType(url)
                                                                    .host}
                                                            </span>
                                                        </div>
                                                        <h3
                                                            class="text-sm sm:text-base font-semibold truncate"
                                                        >
                                                            {makeTitle(url)}
                                                        </h3>
                                                        <p
                                                            class="mt-1 text-xs text-zinc-600 dark:text-zinc-400 truncate"
                                                        >
                                                            {url}
                                                        </p>
                                                    </div>
                                                    <div
                                                        class="shrink-0 flex items-center gap-1"
                                                    >
                                                        <button
                                                            type="button"
                                                            title="Copy link"
                                                            on:click|preventDefault={() =>
                                                                copy(url)}
                                                            class="cursor-pointer px-2 py-1 rounded-lg border border-zinc-300/70 dark:border-zinc-700/80 text-[10px]"
                                                        >
                                                            Copy
                                                        </button>
                                                        <svg
                                                            class="h-5 w-5 opacity-70"
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            viewBox="0 0 24 24"
                                                        >
                                                            <path
                                                                fill="currentColor"
                                                                d="M14 3h7v7h-2V6.41l-9.29 9.3-1.42-1.42 9.3-9.29H14V3z"
                                                            />
                                                        </svg>
                                                    </div>
                                                </div>
                                            </a>
                                        {/each}
                                    </div>
                                {/if}
                            </section>
                        {/if}
                    {/key}
                {/if}
            {/each}
        </div>
    </main>

    <!-- Footer -->
    <footer
        class="mx-auto max-w-7xl px-6 pb-10 text-xs text-zinc-600 dark:text-zinc-400"
    >
        &COPY; <span class="font-semibold">2025 inteniquetic.com</span>
    </footer>
</div>

<style>
    .glass {
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
    }
    .card-hover {
        transition:
            transform 0.2s ease,
            box-shadow 0.2s ease,
            border-color 0.2s ease;
    }
    .card-hover:hover {
        transform: translateY(-2px);
    }
    .gradient-hero {
        background-image:
            radial-gradient(
                72rem 40rem at 0% -10%,
                rgba(99, 102, 241, 0.35),
                transparent 60%
            ),
            radial-gradient(
                56rem 40rem at 100% 0%,
                rgba(56, 189, 248, 0.35),
                transparent 60%
            ),
            radial-gradient(
                40rem 24rem at 50% 110%,
                rgba(34, 197, 94, 0.25),
                transparent 60%
            );
    }
</style>
