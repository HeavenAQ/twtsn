use crate::components::go_back::GoBack;
use crate::pages::not_found::NotFound;
use std::collections::HashMap;

use crate::api::cases::{Case, CaseContent, CaseMetaData, CaseType};
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;

type Cases = HashMap<CaseType, Vec<Case>>;

#[component]
pub fn Case(cx: Scope, case_type: CaseType) -> impl IntoView {
    let cases = use_context::<Resource<(), Result<Cases, ServerFnError>>>(cx)
        .expect("unable to find context");
    view! { cx,
            <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-8 mx-auto w-full animate-fade-down">
                <Transition
                    fallback=move || view! { cx, <span class="loading loading-lg"></span> }
                >
                    {move || {
                        cases
                            .read(cx)
                            .map(|cases| match cases {
                                Ok(cases) => {
                                    cases
                                        .get(&case_type)
                                        .expect("Unable to read the right case_type")
                                        .iter()
                                        .map(|case| {
                                            view! { cx,
                                                <CaseCard
                                                    case_metadata=case.case_metadata.clone()
                                                    path=case.case_metadata.tag.clone()
                                                />
                                            }
                                        }).collect_view(cx)
                                },
                                Err(e) => {
                                    view! { cx,
                                        <pre class="error">"Server Error: " {e.to_string()}</pre>
                                    }.into_view(cx)
                                }
                            })
                    }}
                </Transition>
            </div>
    }
}

#[component]
pub fn CaseCard(cx: Scope, case_metadata: CaseMetaData, path: String) -> impl IntoView {
    view! { cx,
        <a
            href=format!("/cases/{}/{}", path + "s", case_metadata.create_href())
        >
            <div class="card w-auto shadow-xl hover:-translate-y-1 hover:shadow-lg transition-all duration-200 ease-in-out active:translate-y-0 active:shadow-2xl">
              <figure><img src=case_metadata.image alt="case image" /></figure>
              <div class="card-body">
                <h2 class="card-title">
                    {case_metadata.title}
                </h2>
                <div class="badge badge-neutral ">{case_metadata.date}</div>
                <p>{case_metadata.description}</p>
                <div class="card-actions justify-end">
                  <div class="badge badge-outline">{move || CaseType::from(&case_metadata.tag).to_tag_name(cx)}</div>
                </div>
              </div>
            </div>
        </a>
    }
}

#[component]
pub fn RenderCase(cx: Scope, case_type: CaseType) -> impl IntoView {
    let cases = use_context::<Resource<(), Result<Cases, ServerFnError>>>(cx)
        .expect("unable to find context");
    let params = use_params_map(cx);
    let case_query = move || params.with(|params| params.get("case").cloned().unwrap_or_default());

    view! { cx,

        <Suspense fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            {move || {
               cases
                    .read(cx)
                    .map(|cases| match cases {
                        Ok(cases) => {
                            let case = cases
                                .get(&case_type)
                                .expect("Unable to read the right case_type")
                                .iter()
                                .find(|&p| p.case_metadata.create_href() == case_query());

                            match case {
                                Some(case) => {
                                    view! { cx,
                                        <Title text=case.case_metadata.title.clone()/>
                                        <Meta name="description" content=case.case_metadata.description.clone()/>
                                        <CaseLayout content=case.case_content.clone()/>
                                    }.into_view(cx)
                                }
                                None => {
                                    view! { cx, <NotFound /> }.into_view(cx)
                                }
                            }
                        }
                        Err(e) => {
                            view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre> }
                                .into_view(cx)
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn CaseLayout(cx: Scope, content: CaseContent) -> impl IntoView {
    view! { cx,

        <div>
            <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto mt-20 animate-fade-down">
                <div class="max-w-3xl">
                    <div
                        class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none"
                        inner_html=content
                    ></div>
                </div>
            </div>
            <GoBack content="Back to Cases".to_string() href="/cases/all".to_owned()/>
        </div>
    }
}
