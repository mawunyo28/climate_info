use dioxus::prelude::{rsx, *};
const DASHBOARD_ASSET: Asset = asset!("/assets/styling/dashboard.css");
const GAS_EMISSION_1: Asset = asset!("/assets/images/gas_emission1.jpg");
const GAS_EMISSION_2: Asset = asset!("/assets/images/gas_emission2.jpg");
const DEFORESTATION_1: Asset = asset!("/assets/images/deforestation1.jpg");
const DEFORESTATION_2: Asset = asset!("/assets/images/deforestation2.jpg");
const INDUSTRIALISATION_1: Asset = asset!("/assets/images/industrialisation1.jpg");
const INDUSTRIALISATION_2: Asset = asset!("/assets/images/industralisation2.jpg");
const TRANSPORT_1: Asset = asset!("/assets/images/transport1.jpg");
const TRANSPORT_2: Asset = asset!("/assets/images/transport2.jpg");
const AGRICULTURE_1: Asset = asset!("/assets/images/agriculture1.jpg");
const AGRICULTURE_2: Asset = asset!("/assets/images/agriculture2.jpg");

#[component]
pub fn Home() -> Element {
    rsx! {
            document::Link{rel: "stylesheet", href: DASHBOARD_ASSET}
            div {
                class: "flex flex-col items-center justify-center space-y-8 text-center mx-auto max-w-2xl",
                h1 {
                    class: "text-2xl font-bold underline",
                    "The Crisis Of Climate Change"
                },
                blockquote {
                    class: "italic border-l-4 border-gray-500 pl-4",
                    u { "Climate change" },
                    " refers to long-term shifts in temperatures and weather patterns."
                },
                p {
                    class: "max-w-2xl mx-auto",
                    div { "Such shifts can be natural, due to changes in the sun’s activity or large volcanic eruptions." }
                    div { "But since the 1800s, human activities have been the main driver of climate change, primarily due to the burning of fossil fuels like coal, oil and gas." }
                    div { "While climate change includes " u { b { "global warming" } } ", its impacts go far beyond rising temperatures--affecting ecosystems, economics, economies, and human lives globally." }
                    div { "The following sections provide more info about the matter." }
                },
                div {
                    class: "flex flex-col space-y-4",
                    div {
                        class: "causes-section",
                        h2 { class: "text-xl font-semibold", "Causes" },
                        p {
                            div { "Climate change is primarily driven by human activities that increase the concentration of greenhouse gases in the atmosphere." }
                            div { "These gases trap heat, leading to a gradual rise in Earth’s temperature and disrupting climate systems." }
                            div { "The main causes include:" }
                        }
                        img {

                        }
                        h3 { "🌫️ Greenhouse Gas Emissions" }
                        p {
                            div { "Burning fossil fuels for energy and transportation releases large quantities of carbon dioxide (CO₂), methane (CH₄), and nitrous oxide (N₂O)—the key greenhouse gases responsible for global warming." }
                            div { "These gases trap heat in the Earth’s atmosphere, causing a long-term rise in temperatures." }
                        },
                        img {
                            class: "greenhouse-gas",
                            src: "{GAS_EMISSION_1}",
                            alt: "Greenhouse gas emission image 1"
                        },
                        img {
                            class: "greenhouse-gas",
                            src: "{GAS_EMISSION_2}",
                            alt: "Greenhouse gas emission image 2"
                        },
                        h3 { "🌲 Deforestation" }
                        p {
                            div { "Forests act as carbon sinks by absorbing CO₂ from the atmosphere." }
                            div { "When trees are cut down or burned, not only is this stored carbon released, but the Earth also loses one of its most effective tools for balancing the climate." }
                        }
                        img {
                            class: "deforestation",
                            src: "{DEFORESTATION_1}",
                            alt: "Deforestation image 1"
                        }
                        img {
                            class: "deforestation",
                            src: "{DEFORESTATION_2}",
                            alt: "Deforestation image 2"
                        }
                        h3 { "🏭 Industrialization" }
                        p {
                            div { "The rise of industry has brought massive increases in fossil fuel consumption for manufacturing, power generation, and transportation." }
                            div { "Many industrial processes also emit greenhouse gases directly." }
                        }
                        img {
                            class: "industrialization",
                            src: "{INDUSTRIALISATION_1}",
                            alt: "Industrialization image 1"
                        }
                        img {
                            class: "industrialization",
                            src: "{INDUSTRIALISATION_2}",
                            alt: "Industrialization image 2"
                        }
                        h3 { "🚗 Transportation" }
                        p {
                            div { "Vehicles powered by gasoline and diesel emit CO₂ and other pollutants." }
                            div { "As global vehicle ownership increases, so do transportation-related emissions, which now account for around 14–16% of global emissions." }
                        }
                        img {
                            class: "transportation",
                            src: "{TRANSPORT_1}",
                            alt: "Transportation image 1"
                        }
                        img {
                            class: "transportation",
                            src: "{TRANSPORT_2}",
                            alt: "Transportation image 2"
                        }
                        h3 { "🌾 Agriculture" }
                        p {
                            div { "Livestock farming produces methane during digestion, and the use of nitrogen-based fertilizers leads to the release of nitrous oxide." }
                            div { "Additionally, clearing land for agriculture often involves deforestation, further compounding the issue." }
                        }
                        img {
                            class: "agriculture",
                            src: "{AGRICULTURE_1}",
                            alt: "Agriculture image 1"
                        }
                        img {
                            class: "agriculture",
                            src: "{AGRICULTURE_2}",
                            alt: "Agriculture image 2"
                        }
                    },
    div {
        class: "effects-section flex flex-col space-y-4",
        h2 { class: "text-xl font-semibold", "Effects" },
        p {
            div { "The effects of climate change are widespread and worsening." }
            div { "They impact natural ecosystems, human health, economies, and infrastructure." }
            div { "The following are some of the major consequences of a changing climate:" }
        }

        h3 { "🌡️ Rising Temperatures" }
        p {
            div { "Average global temperatures have increased over the past century, with recent years being the hottest on record." }
            div { "This warming alters weather patterns and affects seasonal cycles worldwide." }
        }
        div {
            class: "chart-container temperature-chart",
            
            // You can later mount a temperature chart component here
            // e.g. rsx! { TemperatureChart {} }
        }

        h3 { "🌊 Sea Level Rise" }
        p {
            div { "Melting glaciers and polar ice, along with thermal expansion of seawater, are causing sea levels to rise." }
            div { "Coastal regions face increased flooding, land loss, and saltwater intrusion." }
        }
        div {
            class: "chart-container sea-level-chart",
            // Placeholder for sea level chart visualization
            // e.g. rsx! { SeaLevelChart {} }
        }

        h3 { "⛈️ Extreme Weather" }
        p {
            div { "Climate change is linked to more intense and frequent storms, droughts, and heatwaves." }
            div { "These events cause damage to ecosystems, property, and human lives." }
        }
        div {
            class: "map-container weather-map",
            // You could add an interactive map or image of extreme weather patterns
            // e.g. rsx! { ExtremeWeatherMap {} }
        }

        h3 { "🍽️ Food and Water Insecurity" }
        p {
            div { "Changing precipitation patterns and increased temperatures threaten crop yields and water availability." }
            div { "Communities in vulnerable regions are especially at risk." }
        }
        div {
            class: "chart-container food-security-chart",
            // Optional: insert a crop yield trend chart or food production index
        }

        h3 { "🧍‍♂️ Human Health and Migration" }
        p {
            div { "Rising temperatures and environmental degradation contribute to health issues and force migration." }
            div { "Many populations are being displaced due to unlivable conditions." }
        }
        div {
            class: "chart-container migration-map",
            // Optional: embed heat maps or migration flow visualizations
        }
    },
    div {
        class: "solutions-section flex flex-col space-y-4",
        h2 { class: "text-xl font-semibold", "Solutions" },
        p {
            div { "Addressing climate change requires coordinated action at every level of society." }
            div { "Efforts must focus on reducing emissions, protecting natural systems, and supporting communities." }
        }

        h3 { "🔋 Clean Energy Transition" }
        p {
            div { "Replacing fossil fuels with renewable energy sources is critical to reducing greenhouse gas emissions." }
            div { "Advancements in technology support the shift toward cleaner energy systems." }
        }
        div {
            class: "chart-container energy-transition-chart",
            // Future component: renewable energy usage graph, CO₂ emissions reduction trends
            // e.g. rsx! { EnergyTransitionChart {} }
        }

        h3 { "🌱 Nature-Based Solutions" }
        p {
            div { "Protecting forests, restoring wetlands, and promoting sustainable land use help absorb carbon from the atmosphere." }
            div { "These strategies also enhance biodiversity and resilience." }
        }
        div {
            class: "visual-container reforestation-map",
            // Future component: satellite data maps, forest recovery dashboards
            // e.g. rsx! { ForestRestorationMap {} }
        }

        h3 { "📜 Policy and Global Agreements" }
        p {
            div { "International cooperation and strong environmental policies are essential to driving climate action." }
            div { "Agreements like the Paris Accord aim to limit global warming and promote adaptation." }
        }
        

        h3 { "💡 Individual and Community Action" }
        p {
            div { "Personal choices and local initiatives play a key role in shaping sustainable practices." }
            div { "Every action counts in the collective effort to combat climate change." }
        }
        div {
            class: "interactive-container community-impact",
            // Future component: infographic or slider showing impact of small lifestyle changes
            // e.g. rsx! { CommunityActionImpact {} }
        }
    }


                }
            }
        }
}
