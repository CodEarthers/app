let waterScale = 0.001;
let timeScale = 1;
function NumberUpdate() {
    let waterValue = document.getElementById('water_number_range').value;
    let endScore = waterValue * waterScale * timeScale;
    document.getElementById('water_amount').innerHTML = parseFloat(endScore.toFixed(3)) + 'm<sup>3</sup>';
    document.getElementById('water_amount_L').innerHTML = (endScore * 1000).toFixed() + 'L';
    return usage = endScore.toFixed(3);
}

function ButtonChange() {
    document.activeElement.className == 'button false' ? document.activeElement.className = 'button true' : document.activeElement.className = 'button false';
    CheckIfRight();
}

function UpdateSliderRan() {
    if (document.activeElement.id == 'water_number_range')
    {
        let slider = document.getElementById('water_number_range');
        let value = (slider.value / slider.getAttribute('max')) * 100; //wartosc koncowa, balans wartosci
        document.getElementById('water_bar').style.backgroundImage = `linear-gradient(transparent ${100 - value}%, cornflowerblue ${100 - value}%, midnightblue)`; //procent zapelnienia tla po zbalanowaniu przez value
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
}

function CheckIfRight() {
    if (document.getElementById('water_amount').innerHTML != '0m<sup>3</sup>' && document.getElementsByClassName('button true').length >= 2)
        {
            document.getElementById('potwierdzenie').removeAttribute('disabled');
            document.getElementById('potwierdzenie').value = 'Potwierdź';
        }
        else
        {
            document.getElementById('potwierdzenie').setAttribute('disabled', true);
            document.getElementById('potwierdzenie').value = 'Wymagane Dane';
        }
}

function Submit() {
    answer = confirm("Czy jesteś pewien?");
    if (answer == true)
    {
        document.getElementById('potwierdzenie').value = '';
        document.getElementById('potwierdzenie').toggleAttribute('disabled');
        document.getElementById('result').style.display = 'flex';
        document.getElementById('stats-inactive').style.display = 'flex';
        Podsumowanie();
        AVG();
        scrollBy(0, 1000);
    }
}

function Podsumowanie() {
    let cena = Number(prompt("Cena wody:"));
    let result = (cena * usage).toFixed(2);
    document.getElementById('water_price').innerHTML = `Aktualna cena wody wynosi: ${cena}zl/m<span class="super">3</span>`;
    document.getElementById('usage').innerHTML = `Twoje zużycie wody wynosi: ${cena}zl/m<span class="super">3</span> * ${usage}m<span class="super">3</span> ≈ ${result}zl (${timeScale == 1 ? 'dzien' : timeScale == 30 ? 'miesiac' : 'rok'})`;
    return result;
}

function AVG() {
    let average = 30;
    document.querySelector('#mp_1').style.transform = `rotate(${usage/average*90}deg)`;
    document.querySelector('#mp_2').style.transform = `rotate(${usage/average*90}deg)`;
    document.querySelector('#mp_3').style.transform = `rotate(${usage/average*90}deg)`;
}