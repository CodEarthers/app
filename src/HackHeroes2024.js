let waterScale = 0.001;
let timeScale = 1;
function NumberUpdate() {
    let waterValue = document.querySelector('#water_number_range').value;
    let endScore = waterValue * waterScale * timeScale;
    document.querySelector('#water_amount').innerHTML = parseFloat(endScore.toFixed(3)) + 'm<span class="super">3</span>';
    document.querySelector('#water_amount_L').innerHTML = (endScore * 1000).toFixed() + 'L';
    return usage = parseFloat(endScore.toFixed(3));
}

function ButtonChange() {
    document.activeElement.className == 'button false' ? document.activeElement.className = 'button true' : document.activeElement.className = 'button false';
    CheckIfRight();
}

function UpdateSliderRan() {
    if (document.activeElement.id == 'water_number_range')
    {
        let slider = document.querySelector('#water_number_range');
        let value = (slider.value / slider.getAttribute('max')) * 100; //wartosc koncowa, balans wartosci
        document.querySelector('#water_bar').style.backgroundImage = `linear-gradient(transparent ${100 - value}%, cornflowerblue ${100 - value}%, midnightblue)`; //procent zapelnienia tla po zbalanowaniu przez value
        NumberUpdate(); //wyswietlenie liczby
        CheckIfRight(); //sprawdzenie wymagan
        
        //bugfix niedociagniecia zapelnienia paska
        value <= 50 ? slider.style.backgroundImage = `linear-gradient(to right, midnightblue, cornflowerblue ${value + 0.5}%, white ${value + 0.5}%, cornflowerblue)`
        : slider.style.backgroundImage = `linear-gradient(to right, midnightblue, cornflowerblue ${value - 0.5}%, white ${value - 0.5}%, cornflowerblue)`;
    }
}

function ScaleChange() {
    switch(document.activeElement.id)
    {
        case '1':
            waterScale = 0.001;
            NumberUpdate()
            break;
        case '2':
            waterScale = 0.1;
            NumberUpdate()
            break;
        case '3':
            waterScale = 1;
            NumberUpdate()  
            break;
    }
    switch(document.activeElement.id)
    {
        case 'day':
            timeScale = 1;
            NumberUpdate()
            break;
        case 'month':
            timeScale = 30;
            NumberUpdate()
            break;
        case 'year' :
            timeScale = 365;
            NumberUpdate()
            break;
    }
    CheckIfRight();
}

function CheckIfRight() {
    if (document.querySelector('#water_amount').innerHTML != '0m<span class="super">3</span>' && document.getElementsByClassName('button true').length >= 2)
        {
            document.querySelector('#potwierdzenie').removeAttribute('disabled');
            document.querySelector('#potwierdzenie').value = 'Potwierdź';
        }
        else
        {
            document.querySelector('#potwierdzenie').setAttribute('disabled', true);
            document.querySelector('#potwierdzenie').value = 'Wymagane Dane';
        }
}

function Submit() {
    answer = confirm("Czy jesteś pewien?");
    if (answer == true)
    {
        document.querySelector('#potwierdzenie').value = '';
        document.querySelector('#potwierdzenie').toggleAttribute('disabled');
        document.querySelector('#result').style.display = 'flex';
        document.querySelector('#stats-inactive').style.display = 'flex';
        scrollBy(0, 1000);
        Podsumowanie();
        AVG();
    }
}

function Podsumowanie() {
    let cena = 6;
    let result = (cena * usage).toFixed(2);
    document.querySelector('#water_price').innerHTML = `Aktualna cena wody wynosi ok. ${cena}zł/m<span class="super">3</span>`;
    document.querySelector('#usage').innerHTML = `Twój koszt wody wynosi ok. ${cena}zł/m<span class="super">3</span> * ${usage}m<span class="super">3</span> ≈ ${result}zł (${timeScale == 1 ? 'dzień' : timeScale == 30 ? 'miesiąc' : 'rok'})`;
    return result;
}

function AVG() {
    let average1 = 0.092;
    let average2 = 4;
    let average3 = 34;

    switch (timeScale)
    {
        case 1:
            usage/average1*90 > 180 ? document.querySelector('#mp_1').style.transform = `rotate(180deg)` : document.querySelector('#mp_1').style.transform = `rotate(${usage/average1*90}deg)`;
            (usage*30)/average2*90 > 180 ? document.querySelector('#mp_2').style.transform = `rotate(180deg)` : document.querySelector('#mp_2').style.transform = `rotate(${(usage*30)/average2*90}deg)`;
            (usage*365)/average3*90 > 180 ? document.querySelector('#mp_3').style.transform = `rotate(180deg)` : document.querySelector('#mp_3').style.transform = `rotate(${(usage*365)/average3*90}deg)`;
            break;
        case 30:
            (usage/30)/average1*90 > 180 ? document.querySelector('#mp_1').style.transform = `rotate(180deg)` : document.querySelector('#mp_1').style.transform = `rotate(${(usage/30)/average1*90}deg)`;
            usage/average2*90 > 180 ? document.querySelector('#mp_2').style.transform = `rotate(180deg)` : document.querySelector('#mp_2').style.transform = `rotate(${usage/average2*90}deg)`;
            (usage*12.166)/average3*90 > 180 ? document.querySelector('#mp_3').style.transform = `rotate(180deg)` : document.querySelector('#mp_3').style.transform = `rotate(${(usage*12.166)/average3*90}deg)`;
            break;
        case 365:
            (usage/365)/average1*90 > 180 ? document.querySelector('#mp_1').style.transform = `rotate(180deg)` : document.querySelector('#mp_1').style.transform = `rotate(${(usage/365)/average1*90}deg)`;
            (usage/12.166)/average2*90 > 180 ? document.querySelector('#mp_2').style.transform = `rotate(180deg)` : document.querySelector('#mp_2').style.transform = `rotate(${(usage/12.166)/average2*90}deg)`;
            usage/average3*90 > 180 ? document.querySelector('#mp_3').style.transform = `rotate(180deg)` : document.querySelector('#mp_3').style.transform = `rotate(${usage/average3*90}deg)`;
    }
}
