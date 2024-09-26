use super::switcher::TabSwitcher;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home() -> Html {
    html! {
        <body>
        <h1 class="heading">{"burgburg.net"}</h1>
        <div class="main">
            <div class="outerboxes">
                <div class="innerboxes">

                    <h1>
                        {"Burg's Room"}
                    </h1>

                    /*<img src="https://burgburg.net/indexassets/reader.png"
                     style="width: 21%; float: left; border: 3px solid #44415a; background-color: #8ea48e; margin-right: 6px; margin-left: 19px;"/>*/

                    <p style="align-items: center; display: inline">
                        <br/>
                        {"Welcome to my room! my personal website & blog to hold my web experiences and other fun things."}
                        <br />{"I love to drink Coffee with Boba, and many different Hot Teas (my fav is English Breakfast)."}
                        <br />{"Oatmeal is the best breakfast for when i eat breakfast."}
                        <br />{"Forever eepy."}
                        <br />
                        <br />
                    </p>

                    <h3>
                        {"A Little About Me:"}
                    </h3>
                    <p>
                        {"Im from the Northeast USA 🇺🇸."}
                        <br />
                      {"  Im "}<i>{"trying"}</i> {" to learn Dutch 🇳🇱."}
                        <br />
                        {"My Favorite colors are"} <span style="color: #2d5a29">{"Green"}</span>{", "} <span
                            style="color: #f5acba; text-shadow: 1px 1px 2px black;">{"Pink"}</span>{", and "} <span
                            style="color: #C08BF8; text-shadow: 1px 1px 2px black;">{"Purple"}</span>
                        <br />
                        {"I do some 3D art in "}<a href="https://www.blender.org/">{"Blender"}</a>{" and 2D art in "}<a
                            href="https://krita.org/en/">{"Krita"}</a>
                        <br />
                        {"My Main Coding Laguage is Rust"}
                        <br/>
                        {"I daily drive a Win 10 PC (i9 9900k, gtx 1080, 48 gb ram)"}
                        <br/>
                        {"My laptop is dell inspiron 3537 running Pop!_OS (debian) with the KDE Plasma WM"}

                    </p>
                    <h3>
                       {" My Favorite Shows:"}
                    </h3>
                    <p>
                        {"Girls Band Cry, "}<br />
                        {"Lucky Star, "}<br />
                        {"Bocchi The Rock,"}<br />
                        {"Serial Expiriments Lain,"}<br />
                        {"Kill la Kill,"}<br />
                        {"Better Call Saul,"} <br />
                        {"Madoka Magica"}
                    </p>
                    <h3>
                        {"My Favorite Games:"}
                    </h3>
                    <p>
                        {"Vicky 3,"}<br />
                        {"Escape from Tarkov,"}<br />
                        {"VRChat,"}<br />
                        {"Persona (series),"}<br />
                        {"Hoi4 (i know i know),"}<br />
                        {"Terraria,"}<br />
                        {"Half-Life (series),"}<br />
                        {"Arma Reforger (i never got into 3),"}<br />
                        {"PSO2:NGS"}
                    </p>
                </div>
                <div class="inbetween">
                    <marquee behavior="scroll" direction="left">
                        <span>{"ALL are welcome. Enjoy Your Stay!"}</span>
                    </marquee>
                </div>
                <div class="innerboxes">
                    <h3>
                        {"Projects:"}
                    </h3>
                    <p>
                        {"Custom home server stuff on an old pc using Proxmox. (stuff like an MC server, pihole, etc.)"}
                        <br />
                        {"Learning and using Rust, web dev stuff"}
                    </p>
                </div>
                <div class="inbetween">
                    <br />
                </div>
                <div class="innerboxes">
                    <h3>
                        {"QOTR (quote of the refresh):"}
                    </h3>
                    <p ><QOTR/></p>
                </div>


            </div>

            <div class="outerboxes" style="width: 25%">
                <div class="innerboxes">
                    <h3>{"Links:"}</h3>
                    <p>
                        <a href="https://twitter.com/_burgburg_" target="_blank">{"Twitter"}</a>
                    </p>
                    <p>
                        <a href="https://bsky.app/profile/burgburg.net" target="_blank">{"Bluesky"}</a>
                    </p>
                    <p>
                        <a href="https://steamcommunity.com/id/_burgburg_/" target="_blank">{"Steam"}</a>
                    </p>
                    <p>
                        <a href="https://github.com/BRBergman" target="_blank">{"Github"}</a>
                    </p>
                </div>
                <div class="inbetween">
                    <br />
                </div>
                <div class="innerboxes">
                    <h3>
                        {"Friens:"}
                    </h3>
                    <p>
                        <a href="https://natgeo.nekoweb.org/" target="_blank">{"Natgeo"}</a>
                    </p>
                    <p>
                        <a href="https://swiftersweeper.nekoweb.org/" target="_blank">{"Swift"}</a>
                    </p>
                </div>
                <div class="inbetween">
                    <br />
                </div>
                <div class="innerboxes">

                    
                    <TabSwitcher />
                    
                </div>
                <div class="inbetween">
                    <br />
                </div>
                <div class="innerboxes">
                    <div style="text-align: center;">
                        <img src="https://mypillowfort.nekoweb.org/media5/tumblr_ace137e4afe10e9090c839f452444524_b175a197_75.gif"
                            style="image-rendering: pixelated;"/>
                        <img src="https://mypillowfort.nekoweb.org/media5/tumblr_75f0b86c777ae5383ec55e4aaab64603_f6b25393_75.gif"
                            style="image-rendering: pixelated;"/>
                        <img src="https://mypillowfort.nekoweb.org/media5/tumblr_9d8e9f7bea9bc75fa560df7e001c9f41_1f93276b_75.gif"
                            style="image-rendering: pixelated;"/>
                    </div>
                </div>
                <div class="inbetween"> <br /></div>
                <div class="innerboxes">
                    <img src="https://burgburg.net/indexassets/Lain.jpg" style="object-fit: contain; width: 100%; height: 100%;"/>
                    <marquee behavior="scroll" direction="left">
                        {"Lets All Love Lain."}
                    </marquee>
                </div>
                <div class="inbetween">
                    <br />
                </div>
                <div class="innerboxes">
                    <p1 style="color: #bda4e5; text-shadow: 1px 1px 2px #44415a;">{"awawawawawaw"}</p1>
                    <img src="https://burgburg.net/indexassets/luckyScream.jpg"
                        style="text-align: center; width: 100%; image-rendering: smooth;"/>
                    <div style="text-align: right">
                        <p1 style="color: #748fdc; text-shadow: 1px 1px 2px #52505f;">{"ehe"}</p1>
                    </div>
                </div>
            </div>
        </div>
        <p style="text-align: center; margin-bottom: 0%;">
            <span
                style="font-family: Impact,Haettenschweiler,Franklin Gothic Bold,Charcoal,Helvetica Inserat,Bitstream Vera Sans Bold,Arial Black,sans serif; color: black;">
                {"Bottom Text"}</span>
        </p>
        <script src="/htmx/htmx.min.js"></script>
        <script src="/index.js"></script>
    </body>
    }
}
use rand::Rng;

#[function_component(QOTR)]
fn qotr()-> Html
{
    let file: Vec<&str> = vec!["bwaaa", "DO NOT THROW SOULS", "you're out of touch","haiii"];
    let rand = rand::thread_rng().gen_range(0..file.len());
    return Html::from(file[rand]);
}