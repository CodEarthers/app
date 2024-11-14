import { load } from '@tauri-apps/plugin-store';

(async ()=>{
    store = await load('store.json', { autoSave: true });
    generateAqi()
})()

const generateAqi = () => {
    console.log("generate Aqi")
    invoke('get_aqi', { sensor_id:parseInt(getStation()||0) }).then((msg)=>{
        let data = document.querySelector("#aqiView")
        let list = JSON.parse(msg)
        let tem = '<ons-card>'
        list.forEach(el => {
            tem += `<p>${el}: ${list[el]} </p>`
        })
        tem += '</ons-card>'
        let x = create(tem)
        console.log(x)
        stations.appendChild(x)
    })
}