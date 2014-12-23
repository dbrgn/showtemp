extern crate hyper;

use std::os;
use std::io::stdout;
use std::io::util::copy;

use hyper::Client;


fn main() {
    // Argument parsing
    let args = os::args();
    match args.len() {
        2 => (),
        _ => {
            println!("Usage: showtemp <station>");
            println!("Stations:");
            println!("ABO (Adelboden), AIG (Aigle), ALT (Altdorf), AND (Andeer), \
                      ATT (Les Attelas), BAS (Basel / Binningen), BER (Bern / Zollikofen), \
                      BEZ (Beznau), BIE (Bière), BIZ (Bischofszell), BOL (Boltigen), \
                      BOU (Bouveret), BRZ (Brienz), BUF (Buffalora), BUS (Buchs / Aarau), \
                      CDF (La Chaux-de-Fonds), CGI (Nyon / Changins), CHA (Chasseral), \
                      CHD (Château-d'Oex), CHU (Chur), CIM (Cimetta), CMA (Crap Masegn), \
                      COM (Acquarossa / Comprovasco), COV (Piz Corvatsch), CRM (Cressier), \
                      DAV (Davos), DEM (Delémont), DIS (Disentis / Sedrun), DOL (La Dôle), \
                      EBK (Ebnat-Kappel), EGH (Eggishorn), EGO (Egolzwil), EIN (Einsiedeln), \
                      ELM (Elm), ENG (Engelberg), EVO (Evolène / Villa), FAH (Fahy), \
                      FRE (Bullet / La Frétaz), GEN (Monte Generoso), GIH (Giswil), GLA (Glarus), \
                      GOE (Gösgen), GOR (Gornergrat), GRA (Fribourg / Posieux), GRE (Grenchen), \
                      GRH (Grimsel Hospiz), GRO (Grono), GSB (Col du Grand St-Bernard), \
                      GUE (Gütsch ob Andermatt), GUT (Güttingen), GVE (Genève-Cointrin), \
                      HAI (Salen-Reutenen), HLL (Hallau), HOE (Hörnli), INT (Interlaken), \
                      JUN (Jungfraujoch), KLO (Zürich / Kloten), KOP (Koppigen), LAE (Lägern), \
                      LAG (Langnau i.E.), LEI (Leibstadt), LUG (Lugano), LUZ (Luzern), \
                      MAG (Magadino / Cadenazzo), MER (Meiringen), MLS (Le Moléson), MOA (Mosen), \
                      MOE (Möhlin), MRP (Monte Rosa-Plattje), MUB (Mühleberg), MVE (Montana), \
                      NAP (Napf), NAS (Naluns / Schlivera), NEU (Neuchâtel), ORO (Oron), \
                      OTL (Locarno / Monti), PAY (Payerne), PIL (Pilatus), PIO (Piotta), \
                      PLF (Plaffeien), PMA (Piz Martegnas), PRE (St-Prex), PUY (Pully), \
                      QUI (Quinten), RAG (Bad Ragaz), REH (Zürich / Affoltern), \
                      ROB (Poschiavo / Robbia), ROE (Robièi), RUE (Rünenberg), SAE (Säntis), \
                      SAM (Samedan), SBE (S. Bernardino), SBO (Stabio), SCM (Schmerikon), \
                      SCU (Scuol), SHA (Schaffhausen), SIO (Sion), SMA (Zürich / Fluntern), \
                      SMM (Sta. Maria, Val Müstair), SPF (Schüpfheim), STG (St. Gallen), \
                      STK (Steckborn), TAE (Aadorf / Tänikon), THU (Thun), TIT (Titlis), \
                      ULR (Ulrichen), VAB (Valbella), VAD (Vaduz), VIS (Visp), WAE (Wädenswil), \
                      WFJ (Weissfluhjoch), WYN (Wynau), ZER (Zermatt)");
            return;    
        }
    };
    let station = &*args[1];

    // Instantiate a HTTP client
    let mut client = Client::new();

    // Connect to server
    let url = format!("http://data.netcetera.com/smn/smn/{}/", station);
    let mut res = match client.get(url.as_slice()).send() {
        Ok(res) => res,
        Err(err) => panic!("Failed to connect: {}", err)
    };

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    match copy(&mut res, &mut stdout()) {
        Ok(..) => (),
        Err(e) => panic!("Stream failure: {}", e)
    };
}
