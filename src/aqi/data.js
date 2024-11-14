import { load } from '@tauri-apps/plugin-store';

(async ()=>{
    store = await load('store.json', { autoSave: true });
    generateData()
})()

const getSensor = () => {
    store.get('sensor-id')
}

const generateData = () => {
    console.log("generate Data")
    invoke('get_data', { sensor_id:parseInt(getSensor()||0) }).then((msg)=>{
        let data = document.querySelector("#dataView")
        let list = JSON.parse(msg)
        list.forEach(el => {
            let Data = el["Data"]
            let KodStanowiska = el["Kod stanowiska"]
            let Wartosc = el["Wartość"]

            let tem = `
                <ons-card>
                    <p>Data: ${Data}</p>
                    <p>Kod stanowiska: ${KodStanowiska}</p>
                    <p>Wartość ${Wartosc}</p>
                </ons-card>
            `
            let x = create(tem)
            console.log(x)
            stations.appendChild(x)
        })
    })
}