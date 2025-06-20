use serenity::all::ActivityData;
use std::sync::LazyLock;

pub static LIST: LazyLock<Vec<ActivityData>> = LazyLock::new(|| {
    use ActivityData as ACT;
    vec![
        ACT::playing("Team Fortress 2"),
        ACT::playing("Minecraft"),
        ACT::playing("PGTF Dating Sim"),
        ACT::playing("Hades"),
        ACT::playing("Bloons TD 6"),
        // // // // // // // // // // //
        ACT::listening("Lin-Manuel Miranda"),
        ACT::listening("Kendrick Lamar"),
        ACT::listening("Pięć Dwa Dębiec"),
        ACT::listening("Gimpson"),
        // // // // // // // // // // //
        ACT::watching("Scooby Doo"),
        ACT::watching("Horimiya"),
        ACT::watching("My Deer Friend Nokotan"),
        ACT::watching("Lycoris Recoil"),
        ACT::watching("Yuru Camp"),
        ACT::watching("DARLING in the FRANXX"),
        // // // // // // // // // // //
        ACT::custom("Formalizuje stowarzyszenie"),
        ACT::custom("Shipuje członków"),
        ACT::custom("Spisuje cytaty"),
        ACT::custom("Krzyczy na rozmówcę"),
        ACT::custom("Montuje konwent"),
        ACT::custom("Rozmontowuje konwent"),
        ACT::custom("Głosi prelekcję"),
        ACT::custom("Zmienia nicki innym"),
    ]
});
