
Let's Play With Procedural Macro

L'obbiettivo di questo esercizio e' di implementare un piccolo clone di pytest usando le procedural_macro_attribute messe a disposizione come unstable feature della nightly #[feature(proc_macro)] .

In main.rs si trova il codice che chiama la macro procedurale hello che puo' essere usata come punto di partenza per l'implementazione.

Cosa fare

Creare un nuovo sotto crate nel workspace dove implemetare rstest e ricordarsi di aggiungere

[lib]
proc-macro = true



In questa libbreria bisognera' implementare una funzione pubblica rstest come macro procedurale

#[proc_macro_attribute]
pub fn rstest(_attrs: TokenStream, body: TokenStream) -> TokenStream{
    ...
}



in grado di trasformare il test passato nella nuova che si comporta come

#[test]
fn some_test() {
    let fixture: &str = fixture();
    let fix_string: String = fix_string();

    assert_eq!(fixture, "42");
    assert_eq!(fix_string, "String".to_string());
}



Tips
• La macro quote!{} puo' essere utile per scrivere il risultato finale
• Potrebbe essere piu' semplice lasciare il codice originale in un blocco scritto dopo le variabili

Links
•  proc_macro su unstable book
•  syn crate docs
•  syn repo
•  quote crate docs e quote!() macro
• Presentazione

