import { load } from '@tauri-apps/plugin-store';

(async ()=>{
    store = await load('store.json', { autoSave: true });
    generateSenors()
})()

const setSensor = async (id) => {
    await store.set('sensor-id', id)
}
const getStation = () => {
    store.get('station-id')
}

const generateSenors = () => {
    console.log("generate Sensors")
    invoke('get_sensors', { station_id:getStation() }).then((msg)=>{
        let stations = document.querySelector("#sensorView")
        let list = JSON.parse(msg)
        list.forEach(el => {
            let sensor_id = el["Id wskaźnika"]
            let station_id = el["Identyfikator stacji"]
            let sensor = el["Wskaźnik"]

            let tem = `
                <ons-card id=${sensor_id} onclick="setSensor(this.id)">
                    <p>id wskaźnika: ${sensor_id}</p>
                    <p>identyfikator stacji: ${station_id}</p>
                    <p>wskaźnik: ${sensor}</p>
                </ons-card>
            `
            let x = create(tem)
            console.log(x)
            stations.appendChild(x)
        })
    })
}