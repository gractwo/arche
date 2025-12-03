use serenity::all::ActivityData;
use std::sync::LazyLock;

pub static LIST: LazyLock<Vec<ActivityData>> = LazyLock::new(|| {
    use ActivityData as ACT;
    vec![
        ACT::custom("Playing Team Fortress 2"),
        ACT::custom("Playing Minecraft"),
        ACT::custom("Playing PGTF Dating Sim"),
        ACT::custom("Playing Hades"),
        ACT::custom("Playing Bloons TD 6"),
        // // // // // // // // // // //
        ACT::custom("Listening to Lin-Manuel Miranda"),
        ACT::custom("Listening to Kendrick Lamar"),
        ACT::custom("Listening to Pięć Dwa Dębiec"),
        ACT::custom("Listening to Gimpson"),
        // // // // // // // // // // //
        ACT::custom("Watching Scooby Doo"),
        ACT::custom("Watching Horimiya"),
        ACT::custom("Watching My Deer Friend Nokotan"),
        ACT::custom("Watching Lycoris Recoil"),
        ACT::custom("Watching Yuru Camp"),
        ACT::custom("Watching DARLING in the FRANXX"),
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
