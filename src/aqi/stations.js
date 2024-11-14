import { load } from '@tauri-apps/plugin-store';

let store

(async ()=>{
    store = await load('store.json', { autoSave: true });
    generateStations()
})()

const create = (html) => {
    let template = document.createElement("template")
    template.innerHTML = html.trim()
    return template.content.firstChild
}

const setStation = async (id) => {
    await store.set('station-id', id)
}

const generateStations = () => {
    console.log("generate Stations")
    invoke('find_all').then((msg)=>{
        let stations = document.querySelector("#stationView")
        let list = JSON.parse(msg)
        list.forEach(el => {
            let station_name = el["Nazwa stacji"]
            let station_id = el["Identyfikator stacji"]
            let miasto = el["Gmina"]
            let n = el["WGS84 φ N"]
            let e = el["WGS84 λ E"]

            let tem = `
                <ons-card id=${station_id} onclick="setStation(this.id)">
                    <p>nazwa stacji: ${station_name}</p>
                    <p>miasto: ${miasto}</p>
                    <p>lokalizacja: ${n},${e}</p>
                </ons-card>
            `
            let x = create(tem)
            console.log(x)
            stations.appendChild(x)
        })
    })
}