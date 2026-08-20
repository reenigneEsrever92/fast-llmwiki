use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::{use_location, use_navigate, use_params_map, use_query_map},
    NavigateOptions, SsrMode, StaticSegment, WildcardSegment,
};
use fawi_core::concept::TrustTier;
use fawi_core::dto::{
    ConceptResponse, ConceptSummaryResponse, DirListingResponse, TreeNodeResponse,
};

use crate::api_client::{fetch_page, fetch_search, fetch_tree, PageData};

const STYLE: &str = r#"
:root { --bg:#fff; --fg:#1a1a1a; --muted:#6b7280; --border:#e5e7eb; --accent:#2563eb; --danger:#dc2626; --warn:#b45309; --green:#15803d; }
* { box-sizing: border-box; }
body { margin:0; font-family:system-ui,-apple-system,"Segoe UI",Roboto,sans-serif; color:var(--fg); background:var(--bg); line-height:1.6; }
.site-header { display:flex; align-items:center; gap:1rem; padding:0.75rem 1.25rem; border-bottom:1px solid var(--border); }
.brand { font-weight:700; font-size:1.05rem; color:var(--fg); text-decoration:none; }
.search { flex:1; display:flex; position:relative; }
.search input { width:100%; max-width:26rem; padding:0.4rem 0.75rem; border:1px solid var(--border); border-radius:6px; font-size:0.95rem; }
.search-dropdown { position:absolute; top:calc(100% + 0.35rem); left:0; z-index:600; width:100%; max-width:26rem; margin:0; padding:0; list-style:none; background:var(--bg); border:1px solid var(--border); border-radius:6px; box-shadow:0 8px 24px rgba(0,0,0,0.12); max-height:20rem; overflow-y:auto; }
.search-dropdown li { border-bottom:1px solid var(--border); }
.search-dropdown li:last-child { border-bottom:none; }
.search-dropdown a { display:block; padding:0.5rem 0.75rem; text-decoration:none; color:var(--fg); }
.search-dropdown a:hover { background:#f3f4f6; }
.search-dropdown .title { font-weight:600; font-size:0.9rem; }
.search-dropdown .desc { color:var(--muted); font-size:0.82rem; margin-top:0.1rem; }
.content { flex:1 1 0%; min-width:0; max-width:56rem; margin:0 auto; padding:1.5rem 1.25rem 4rem; }
.muted { color:var(--muted); font-size:0.85rem; }
.badge { display:inline-block; padding:0.1rem 0.5rem; border-radius:9999px; font-size:0.72rem; font-weight:600; }
.badge.type { background:#eef2ff; color:#3730a3; }
.badge.status-stable { background:#ecfdf5; color:var(--green); }
.badge.status-draft { background:#fffbeb; color:var(--warn); }
.badge.status-deprecated { background:#f3f4f6; color:var(--muted); }
.badge.trust-human { background:#ecfdf5; color:var(--green); }
.badge.trust-machine { background:#eff6ff; color:#1d4ed8; }
.badge.trust-unverified { background:#f3f4f6; color:var(--muted); }
.badge.stale { background:#fef2f2; color:var(--danger); }
.badge.tag { background:#f3f4f6; color:var(--muted); font-weight:500; }
.page-head { display:flex; flex-wrap:wrap; align-items:baseline; gap:0.5rem; margin-bottom:0.5rem; }
.page-head h1 { margin:0; font-size:1.6rem; }
.meta { display:flex; flex-wrap:wrap; gap:0.4rem; margin:0.75rem 0; }
.dir-list { display:flex; flex-wrap:wrap; gap:0.5rem; margin:0.75rem 0; }
.dir-list a { padding:0.3rem 0.6rem; border:1px solid var(--border); border-radius:6px; text-decoration:none; color:var(--fg); font-size:0.9rem; }
.concept-list { list-style:none; padding:0; margin:0.75rem 0; }
.concept-list li { padding:0.75rem 0; border-bottom:1px solid var(--border); }
.concept-list a.title { font-weight:600; color:var(--fg); text-decoration:none; }
.concept-list a.title:hover { color:var(--accent); }
.concept-list .desc { color:var(--muted); font-size:0.9rem; margin-top:0.15rem; }
.page-body { overflow-wrap:break-word; }
.page-body pre { background:#f6f8fa; padding:1rem; border-radius:6px; overflow-x:auto; }
.page-body code { font-family:ui-monospace,SFMono-Regular,Menlo,monospace; }
.page-body table { border-collapse:collapse; margin:1rem 0; }
.page-body th, .page-body td { border:1px solid var(--border); padding:0.4rem 0.6rem; }
h2.section { margin-top:2rem; border-bottom:1px solid var(--border); padding-bottom:0.3rem; }
table.src-table { width:100%; border-collapse:collapse; font-size:0.9rem; margin:0.75rem 0; }
table.src-table th, table.src-table td { border:1px solid var(--border); padding:0.4rem 0.6rem; text-align:left; vertical-align:top; }
table.src-table th { background:#f9fafb; }
.site-footer { border-top:1px solid var(--border); padding:1rem 1.25rem; color:var(--muted); font-size:0.85rem; text-align:center; }
.layout { display:flex; flex:1; min-height:0; align-items:stretch; }
.sidebar-toggle { display:none; align-items:center; justify-content:center; width:2.25rem; height:2.25rem; padding:0; border:1px solid var(--border); border-radius:6px; background:var(--bg); color:var(--fg); font-size:1.1rem; cursor:pointer; flex-shrink:0; }
.sidebar { width:16rem; flex-shrink:0; border-right:1px solid var(--border); padding:0.75rem 0; overflow-y:auto; background:var(--bg); }
.tree { padding:0 0.5rem; }
.tree-level { list-style:none; margin:0; padding:0 0 0 1rem; }
.tree > .tree-level { padding-left:0; }
.tree-node { margin:0; }
.tree-row { display:flex; align-items:center; gap:0.25rem; border-radius:6px; }
.tree-row.active { background:#eef2ff; }
.tree-chevron { width:1.25rem; height:1.25rem; flex-shrink:0; display:inline-flex; align-items:center; justify-content:center; border:none; background:transparent; color:var(--muted); cursor:pointer; font-size:0.8rem; padding:0; }
.tree-chevron:empty { cursor:default; }
.tree-link { flex:1; padding:0.25rem 0.4rem; border-radius:6px; color:var(--fg); text-decoration:none; font-size:0.9rem; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
.tree-link:hover { color:var(--accent); }
.tree-link.concept { font-weight:500; }
.tree-row.active .tree-link { color:var(--accent); font-weight:600; }
.toast { position:fixed; bottom:1rem; right:1rem; display:flex; align-items:center; gap:0.75rem; background:#1a1a1a; color:#fff; padding:0.75rem 1rem; border-radius:8px; box-shadow:0 4px 12px rgba(0,0,0,0.2); z-index:1000; }
.toast button { background:var(--accent); color:#fff; border:none; border-radius:6px; padding:0.35rem 0.75rem; cursor:pointer; font-size:0.85rem; }
@media (max-width: 768px) {
  .layout { position:relative; }
  .sidebar { position:fixed; top:3.25rem; bottom:0; left:0; z-index:500; transform:translateX(-100%); transition:transform 0.2s ease; box-shadow:2px 0 8px rgba(0,0,0,0.15); }
  .sidebar.open { transform:translateX(0); }
  .sidebar-toggle { display:inline-flex; }
}
"#;

/// Signal that triggers the sidebar navigation tree to re-fetch on every bundle
/// change. Wrapped in a newtype so [`provide_context`]/[`use_context`] (keyed by
/// type) can tell it apart from [`PageReload`].
#[derive(Copy, Clone)]
struct SidebarReload(RwSignal<u64>);

/// Signal that triggers the currently-viewed page to re-fetch only when its
/// path is affected by a bundle change.
#[derive(Copy, Clone)]
struct PageReload(RwSignal<u64>);

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <style>{STYLE}</style>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let sidebar_open = RwSignal::new(false);
    let reload = SidebarReload(RwSignal::new(0u64));
    provide_context(reload);
    let page_reload = PageReload(RwSignal::new(0u64));
    provide_context(page_reload);

    view! {
        <Router>
            <header class="site-header">
                <button
                    class="sidebar-toggle"
                    aria-label="Toggle navigation"
                    on:click=move |_| sidebar_open.update(|v| *v = !*v)
                >
                    "☰"
                </button>
                <a class="brand" href="/">"OKF Bundle"</a>
                <HeaderSearch/>
            </header>
            <div class="layout">
                <Sidebar open=sidebar_open/>
                <main class="content">
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=StaticSegment("") view=Page ssr=SsrMode::Async/>
                        <Route path=StaticSegment("search") view=Search ssr=SsrMode::Async/>
                        <Route path=WildcardSegment("rest") view=Page ssr=SsrMode::Async/>
                    </Routes>
                </main>
            </div>
            <footer class="site-footer">"OKF Bundle · read-only"</footer>
            <HotReload/>
        </Router>
    }
}

#[component]
fn HeaderSearch() -> impl IntoView {
    let query_map = use_query_map();
    let navigate = use_navigate();

    // The header field is the single source of truth for the query. Initialize
    // it from the URL so it shows the active query on the search page during
    // both SSR and hydration.
    let input = RwSignal::new(query_map.get_untracked().get("q").unwrap_or_default());
    let open = RwSignal::new(false);

    // Keep the field in sync with the URL query on client-side navigation: it
    // clears when leaving a search page and follows deep links when arriving.
    Effect::new(move |_| {
        let q = query_map.get().get("q").unwrap_or_default();
        input.set(q);
    });

    let results = Resource::new(
        move || input.get(),
        move |q| crate::api_client::to_send_future(async move { fetch_search(&q).await }),
    );

    let container = NodeRef::<leptos::html::Form>::new();

    // Close the dropdown when the user clicks anywhere outside the search form.
    let handle = window_event_listener(leptos::ev::click, {
        let container = container;
        let open = open;
        move |event: leptos::web_sys::MouseEvent| {
            let Some(node) = event
                .target()
                .and_then(|t| t.dyn_into::<leptos::web_sys::Node>().ok())
            else {
                return;
            };
            if let Some(el) = container.get_untracked() {
                if !el.contains(Some(&node)) {
                    open.set(false);
                }
            }
        }
    });
    on_cleanup(move || handle.remove());

    let on_input = move |event: leptos::web_sys::Event| {
        let value = event_target_value(&event);
        input.set(value.clone());
        open.set(!value.trim().is_empty());
    };

    let on_keydown = move |event: leptos::web_sys::KeyboardEvent| {
        if event.key() == "Escape" {
            open.set(false);
        }
    };

    let on_submit = move |event: leptos::web_sys::SubmitEvent| {
        event.prevent_default();
        let query = input.get_untracked();
        open.set(false);
        if query.trim().is_empty() {
            return;
        }
        let path = format!("/search?q={}", crate::api_client::urlencode(&query));
        navigate(path.as_str(), NavigateOptions::default());
    };

    view! {
        <form class="search" action="/search" method="get" node_ref=container on:submit=on_submit>
            <input
                type="search"
                name="q"
                placeholder="Search concepts…"
                autocomplete="off"
                value=move || input.get()
                on:input=on_input
                on:keydown=on_keydown
            />
            {move || {
                if !open.get() || input.get().trim().is_empty() {
                    ().into_any()
                } else {
                    match results.get() {
                        Some(items) if !items.is_empty() => {
                            view! {
                                <ul class="search-dropdown">
                                    <For each=move || items.clone().into_iter().take(8) key=|c| c.id.clone() let:item>
                                        <li>
                                            <a href=format!("/{}", item.id.clone())>
                                                <div class="title">{item.title.clone()}</div>
                                                {if let Some(desc) = item.description.clone() {
                                                    view! { <div class="desc">{desc}</div> }.into_any()
                                                } else { ().into_any() }}
                                            </a>
                                        </li>
                                    </For>
                                </ul>
                            }.into_any()
                        }
                        _ => ().into_any(),
                    }
                }
            }}
        </form>
    }
}

#[component]
fn Sidebar(open: RwSignal<bool>) -> impl IntoView {
    let reload = use_context::<SidebarReload>()
        .expect("reload signal not provided")
        .0;

    let tree = Resource::new(
        move || reload.get(),
        move |_| crate::api_client::to_send_future(async move { fetch_tree().await }),
    );

    let location = use_location();
    let pathname = location.pathname;

    let aside_class = move || {
        if open.get() {
            "sidebar open"
        } else {
            "sidebar"
        }
    };

    view! {
        <aside class=aside_class aria-label="Bundle navigation">
            <nav class="tree">
                <Suspense fallback=move || view! { <p class="muted">"Loading…"</p> }>
                    {move || match tree.get() {
                        Some(node) => view! {
                            <ul class="tree-level">
                                <TreeItem node=node pathname=pathname/>
                            </ul>
                        }.into_any(),
                        None => view! {}.into_any(),
                    }}
                </Suspense>
            </nav>
        </aside>
    }
}

#[component]
fn TreeItem(node: TreeNodeResponse, pathname: Memo<String>) -> impl IntoView {
    let is_root = node.path.is_empty();
    let path = node.path.clone();
    let name = if is_root {
        "Home".to_string()
    } else {
        node.name.clone()
    };
    let href = if is_root {
        "/".to_string()
    } else {
        format!("/{path}/")
    };
    let has_children = !node.children.is_empty() || !node.concepts.is_empty();

    let expanded = RwSignal::new(false);

    let active_path = path.clone();
    let is_active = Memo::new(move |_| {
        let cur = pathname.get();
        let cur = cur.trim_matches('/');
        if is_root {
            cur.is_empty()
        } else {
            cur == active_path.as_str()
        }
    });

    let ancestor_path = path.clone();
    let ancestor_prefix = format!("{path}/");
    let is_ancestor = Memo::new(move |_| {
        if is_root {
            return true;
        }
        let cur = pathname.get();
        let cur = cur.trim_matches('/');
        cur == ancestor_path.as_str() || cur.starts_with(ancestor_prefix.as_str())
    });

    let is_open = Memo::new(move |_| expanded.get() || is_ancestor.get());
    let row_class = Memo::new(move |_| {
        if is_active.get() {
            "tree-row active"
        } else {
            "tree-row"
        }
    });

    view! {
        <li class="tree-node">
            <div class=row_class>
                {if has_children && !is_root {
                    view! {
                        <button
                            class="tree-chevron"
                            aria-label="Toggle"
                            on:click=move |_| expanded.update(|v| *v = !*v)
                        >
                            {move || if is_open.get() { "▾" } else { "▸" }}
                        </button>
                    }.into_any()
                } else {
                    view! { <span class="tree-chevron"></span> }.into_any()
                }}
                <a class="tree-link" href=href.clone()>{name}</a>
            </div>
            {move || if is_open.get() {
                let concepts = node.concepts.clone();
                let children = node.children.clone();
                view! {
                    <ul class="tree-level">
                        <For each=move || concepts.clone() key=|c| c.id.clone() let:item>
                            <ConceptLeaf summary=item pathname=pathname/>
                        </For>
                        <For each=move || children.clone() key=|c| c.path.clone() let:child>
                            <TreeItem node=child pathname=pathname/>
                        </For>
                    </ul>
                }.into_any()
            } else {
                ().into_any()
            }}
        </li>
    }
}

#[component]
fn ConceptLeaf(summary: ConceptSummaryResponse, pathname: Memo<String>) -> impl IntoView {
    let id = summary.id.clone();
    let title = summary.title.clone();
    let href = format!("/{id}");

    let row_class = Memo::new(move |_| {
        let cur = pathname.get();
        let cur = cur.trim_matches('/');
        if cur == id.as_str() {
            "tree-row active"
        } else {
            "tree-row"
        }
    });

    view! {
        <li class="tree-node">
            <div class=row_class>
                <span class="tree-chevron"></span>
                <a class="tree-link concept" href=href>{title}</a>
            </div>
        </li>
    }
}

#[component]
fn Page() -> impl IntoView {
    let params = use_params_map();
    let page_reload = use_context::<PageReload>()
        .expect("page reload signal not provided")
        .0;
    let id = move || params.get().get("rest").unwrap_or_default();
    let data = Resource::new(
        move || (id(), page_reload.get()),
        move |(id, _)| crate::api_client::to_send_future(async move { fetch_page(&id).await }),
    );

    // Preserve the window scroll offset across an in-place hot reload of the
    // current page. `page_reload` advances only when `HotReload` decides the
    // current path is affected by a bundle change while the route stays the
    // same; navigation changes the route without touching `page_reload`.
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |prev: Option<(String, u64)>| {
            let cur = (
                params.get().get("rest").unwrap_or_default(),
                page_reload.get(),
            );

            let is_reload = matches!(
                prev,
                Some((prev_id, prev_reload)) if prev_id == cur.0 && cur.1 > prev_reload
            );

            if is_reload {
                if let Some(window) = web_sys::window() {
                    if let Ok(y) = window.scroll_y() {
                        wasm_bindgen_futures::spawn_local(async move {
                            // Wait for the reload's fetch to finish so the offset
                            // is applied to the new content, not the old one.
                            data.ready().await;
                            if let Some(window) = web_sys::window() {
                                request_animation_frame(move || {
                                    let _ = window.scroll_to_with_x_and_y(0.0, y);
                                });
                            }
                        });
                    }
                }
            }

            cur
        });
    }

    view! {
        <Suspense fallback=move || view! { <p class="muted">"Loading…"</p> }>
            {move || match data.get() {
                Some(PageData::Concept(c)) => view! { <ConceptView concept=c/> }.into_any(),
                Some(PageData::Dir(d)) => view! { <DirView dir=d/> }.into_any(),
                Some(PageData::NotFound) => view! { <NotFound/> }.into_any(),
                None => view! {}.into_any(),
            }}
        </Suspense>
    }
}

#[component]
fn Search() -> impl IntoView {
    let query = use_query_map();
    let q = move || query.get().get("q").unwrap_or_default();

    let results = Resource::new(q, move |q| {
        crate::api_client::to_send_future(async move { fetch_search(&q).await })
    });

    view! {
        <nav><a href="/">"Home"</a></nav>
        <h1>"Search"</h1>
        <Suspense fallback=move || view! { <p class="muted">"Searching…"</p> }>
            {move || {
                let items = results.get().unwrap_or_default();
                if items.is_empty() {
                    view! { <p class="muted">"No concepts match."</p> }.into_any()
                } else {
                    view! {
                        <ul class="concept-list">
                            <For each=move || items.clone() key=|c| c.id.clone() let:item>
                                <ConceptListItem summary=item/>
                            </For>
                        </ul>
                    }.into_any()
                }
            }}
        </Suspense>
    }
}

#[component]
fn ConceptListItem(summary: ConceptSummaryResponse) -> impl IntoView {
    let href = format!("/{}", summary.id);
    let title = summary.title;
    let concept_type = summary.concept_type;
    let status = summary.status.as_str();
    let trust = summary.trust_tier.as_str();
    let status_class = format!("badge status-{status}");
    let trust_class_name = format!("badge {}", trust_class(summary.trust_tier));
    let stale = summary.stale;
    let description = summary.description;
    let tags = summary.tags;

    view! {
        <li>
            <div class="page-head">
                <a class="title" href=href>{title}</a>
                <span class="badge type">{concept_type}</span>
                <span class=status_class>{status}</span>
                <span class=trust_class_name>{trust}</span>
                {if stale { view! { <span class="badge stale">"stale"</span> }.into_any() } else { ().into_any() }}
            </div>
            {if let Some(desc) = description {
                view! { <div class="desc">{desc}</div> }.into_any()
            } else { ().into_any() }}
            {if !tags.is_empty() {
                view! {
                    <div class="meta">
                        {tags.into_iter().map(|t| view! { <span class="badge tag">{t}</span> }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else { ().into_any() }}
        </li>
    }
}

#[component]
fn ConceptView(concept: ConceptResponse) -> impl IntoView {
    let title = concept.title;
    let concept_type = concept.concept_type;
    let status = concept.status.as_str();
    let trust = concept.trust_tier.as_str();
    let status_class = format!("badge status-{status}");
    let trust_class_name = format!("badge {}", trust_class(concept.trust_tier));
    let stale = concept.stale;
    let tags = concept.tags;
    let description = concept.description;
    let resource = concept.resource;
    let stale_after = concept
        .stale_after
        .map(|d| d.to_string())
        .unwrap_or_default();
    let generated = concept
        .generated
        .as_ref()
        .map(|g| match g.at {
            Some(at) => format!("{} · {}", g.by, at.format("%Y-%m-%d %H:%M UTC")),
            None => g.by.clone(),
        })
        .unwrap_or_default();
    let verified = concept
        .verified
        .iter()
        .map(|v| match v.at {
            Some(at) => format!("{} · {}", v.by, at.format("%Y-%m-%d %H:%M UTC")),
            None => v.by.clone(),
        })
        .collect::<Vec<_>>();
    let sources = concept.sources;
    let has_trust = !generated.is_empty() || !verified.is_empty() || !stale_after.is_empty();
    let content_html = concept.content_html;

    // Build table rows up front and render them without interleaving whitespace
    // text nodes: the HTML parser foster-parents whitespace out of `<table>`,
    // which would otherwise cause a hydration mismatch.
    let mut trust_rows: Vec<AnyView> = Vec::new();
    if !generated.is_empty() {
        trust_rows.push(view! { <tr><th>"Generated by"</th><td>{generated}</td></tr> }.into_any());
    }
    for v in verified {
        trust_rows.push(view! { <tr><th>"Verified by"</th><td>{v}</td></tr> }.into_any());
    }
    if !stale_after.is_empty() {
        trust_rows.push(view! { <tr><th>"Stale after"</th><td>{stale_after}</td></tr> }.into_any());
    }

    let source_rows: Vec<AnyView> = if sources.is_empty() {
        Vec::new()
    } else {
        let mut rows = vec![view! {
            <tr><th>"ID"</th><th>"Title"</th><th>"Resource"</th><th>"Author"</th><th>"Usage"</th><th>"Modified"</th></tr>
        }
        .into_any()];
        for s in sources {
            let s_id = s.id.unwrap_or_default();
            let s_title = s.title.unwrap_or_default();
            let s_resource = s.resource.unwrap_or_default();
            let s_author = s.author.unwrap_or_default();
            let s_usage = s.usage_count.map(|n| n.to_string()).unwrap_or_default();
            let s_modified = s.last_modified.map(|d| d.to_string()).unwrap_or_default();
            let s_has_resource = !s_resource.is_empty();
            rows.push(view! {
                <tr>
                    <td>{s_id}</td>
                    <td>{s_title}</td>
                    <td>{if s_has_resource { view! { <a href=s_resource.clone()>{s_resource.clone()}</a> }.into_any() } else { ().into_any() }}</td>
                    <td>{s_author}</td>
                    <td>{s_usage}</td>
                    <td>{s_modified}</td>
                </tr>
            }
            .into_any());
        }
        rows
    };

    view! {
        <article>
            <nav><a href="/">"Home"</a></nav>
            <div class="page-head">
                <h1>{title}</h1>
                <span class="badge type">{concept_type}</span>
                <span class=status_class>{status}</span>
                <span class=trust_class_name>{trust}</span>
                {if stale { view! { <span class="badge stale">"stale"</span> }.into_any() } else { ().into_any() }}
            </div>

            {if !tags.is_empty() {
                view! { <div class="meta">{tags.into_iter().map(|t| view! { <span class="badge tag">{t}</span> }).collect::<Vec<_>>()}</div> }.into_any()
            } else { ().into_any() }}

            {if let Some(desc) = description { view! { <p>{desc}</p> }.into_any() } else { ().into_any() }}
            {if let Some(res) = resource { view! { <p class="muted">"Resource: " <a href=res.clone()>{res.clone()}</a></p> }.into_any() } else { ().into_any() }}

            {if has_trust {
                view! {
                    <h2 class="section">"Trust"</h2>
                    <table class="src-table">{trust_rows}</table>
                }.into_any()
            } else { ().into_any() }}

            {if !source_rows.is_empty() {
                view! {
                    <h2 class="section">"Provenance"</h2>
                    <table class="src-table">{source_rows}</table>
                }.into_any()
            } else { ().into_any() }}

            <h2 class="section">"Content"</h2>
            <div class="page-body" inner_html=content_html></div>
        </article>
    }
}

#[component]
fn DirView(dir: DirListingResponse) -> impl IntoView {
    let path = dir.path.clone();
    let breadcrumbs = build_breadcrumbs(&path);
    let index_html = dir.index_html;
    let log_html = dir.log_html;
    let subdirs = dir.subdirs;
    let concepts = dir.concepts;

    view! {
        <div>
            <nav>
                {breadcrumbs.into_iter().map(|(label, href, last)| {
                    if last {
                        view! { <span class="muted">{label}</span> }.into_any()
                    } else {
                        view! { <a href=href>{label}</a> " / " }.into_any()
                    }
                }).collect::<Vec<_>>()}
            </nav>

            {if let Some(html) = index_html {
                view! { <div class="page-body" inner_html=html></div> }.into_any()
            } else { ().into_any() }}

            {if !subdirs.is_empty() {
                view! {
                    <h2 class="section">"Directories"</h2>
                    <div class="dir-list">
                        {subdirs.into_iter().map(|name| {
                            let href = if path.is_empty() { format!("/{name}/") } else { format!("/{path}/{name}/") };
                            view! { <a href=href>{name} "/"</a> }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else { ().into_any() }}

            <h2 class="section">"Concepts"</h2>
            {if concepts.is_empty() {
                view! { <p class="muted">"No concepts in this directory."</p> }.into_any()
            } else {
                view! {
                    <ul class="concept-list">
                        <For each=move || concepts.clone() key=|c| c.id.clone() let:item>
                            <ConceptListItem summary=item/>
                        </For>
                    </ul>
                }.into_any()
            }}

            {if let Some(html) = log_html {
                view! {
                    <h2 class="section">"Log"</h2>
                    <div class="page-body" inner_html=html></div>
                }.into_any()
            } else { ().into_any() }}
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <nav><a href="/">"Home"</a></nav>
        <h1>"Not found"</h1>
        <p>"No concept or directory at this path."</p>
    }
}

#[component]
fn HotReload() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        let reload = use_context::<SidebarReload>()
            .expect("reload signal not provided")
            .0;
        let page_reload = use_context::<PageReload>()
            .expect("page reload signal not provided")
            .0;
        // `HotReload` is rendered outside any matched route, so `use_params_map`
        // would panic during hydration; read the reactive location instead.
        let pathname = use_location().pathname;

        wasm_bindgen_futures::spawn_local(async move {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Ok(origin) = window.location().origin() else {
                return;
            };
            let ws_url = origin.replace("http", "ws") + "/api/ws";
            let Ok(mut ws) = gloo_net::websocket::futures::WebSocket::open(&ws_url) else {
                return;
            };

            use futures::{SinkExt, StreamExt};
            // Watch the bundle root: a single shared connection notifies both the
            // sidebar and the page watcher, and the affected-path list tells us
            // which page (if any) needs to re-fetch.
            let watch = serde_json::json!({ "type": "watch", "path": "" });
            let _ = ws
                .send(gloo_net::websocket::Message::Text(watch.to_string()))
                .await;

            while let Some(Ok(msg)) = ws.next().await {
                let gloo_net::websocket::Message::Text(text) = msg else {
                    continue;
                };
                let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
                    continue;
                };
                if value.get("type").and_then(|t| t.as_str()) != Some("change") {
                    continue;
                }

                // Any bundle change refreshes the sidebar tree.
                reload.update(|v| *v += 1);

                // Refresh the current page only when it is affected.
                let affected: Vec<&str> = value
                    .get("paths")
                    .and_then(|p| p.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();
                let current = pathname.get_untracked();
                if page_is_affected(&current, &affected) {
                    page_reload.update(|v| *v += 1);
                }
            }
        });
    }

    ()
}

/// Whether a change to any of `affected` (paths from `ChangeEvent.paths`)
/// affects the page at `current` (a URL pathname). Mirrors the server's
/// `is_affected` matching so client and server agree on what is "unrelated".
#[cfg(feature = "hydrate")]
fn page_is_affected(current: &str, affected: &[&str]) -> bool {
    let w = current.trim_matches('/');
    affected.iter().any(|changed| {
        let c = changed.trim_matches('/');
        if w.is_empty() {
            return true;
        }
        if w == c {
            return true;
        }
        if c.is_empty() {
            return true;
        }
        w.starts_with(&format!("{c}/")) || c.starts_with(&format!("{w}/"))
    })
}

fn trust_class(tier: TrustTier) -> &'static str {
    match tier {
        TrustTier::Unverified => "trust-unverified",
        TrustTier::MachineConfirmed => "trust-machine",
        TrustTier::HumanReviewed => "trust-human",
    }
}

fn build_breadcrumbs(path: &str) -> Vec<(String, String, bool)> {
    let mut out = vec![("Home".to_string(), "/".to_string(), path.is_empty())];
    if path.is_empty() {
        return out;
    }
    let segments: Vec<&str> = path.split('/').collect();
    let mut acc = String::new();
    for (i, seg) in segments.iter().enumerate() {
        if acc.is_empty() {
            acc = seg.to_string();
        } else {
            acc = format!("{acc}/{seg}");
        }
        out.push((seg.to_string(), format!("/{acc}/"), i == segments.len() - 1));
    }
    out
}
