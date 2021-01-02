export function load_dashboard(data) {
    var ctx = document.getElementById('myChart').getContext('2d');
    var timeFormat = 'YYYY-MM-DD HH:mm:ss ZZ';

    var chart = new Chart(ctx, {
        // The type of chart we want to create
        type: 'bar',

        // The data for our dataset
        data: {
            datasets: [{
                label: data.sensor1.name,
                backgroundColor: 'rgba(255, 0, 0, 0.2)',
                borderColor: 'rgb(255, 0, 0)',
                data: data.sensor1.soil_humidity_readings,
                type: 'line',
                pointRadius: 0,
                fill: false,
                lineTension: 0,
                borderWidth: 2
            },
            {
                label: '💦 ' + data.sensor1.name,
                backgroundColor: 'rgba(255, 0, 0, 0.2)',
                borderColor: 'rgb(255, 0, 0)',
                data: data.sensor1.watering_readings,
                radius: 5,
                hoverRadius: 10,
                type: 'bubble',
            },
            {
                label: data.sensor2.name,
                backgroundColor: 'rgba(0, 0, 255, 0.2)',
                borderColor: 'rgb(0, 0, 255)',
                data: data.sensor2.soil_humidity_readings,
                type: 'line',
                pointRadius: 0,
                fill: false,
                lineTension: 0,
                borderWidth: 2
            },
            {
                label: '💦 ' + data.sensor2.name,
                backgroundColor: 'rgba(0, 0, 255, 0.2)',
                borderColor: 'rgb(0, 0, 255)',
                data: data.sensor2.watering_readings,
                radius: 5,
                hoverRadius: 10,
                type: 'bubble',
            }]
        },

        // Configuration options go here
        options: {
            title: {
                display: true,  
                text: `Plant state ${moment(data.from).fromNow()} - ${moment(data.to).fromNow()}` ,
                fontSize: 24,
                padding: 25
            },
            responsive: true,
            legend: {
                position: 'bottom',
            },
            tooltips: {
                position: 'nearest',
                mode: 'index',
                intersect: false,
            },

            scales: {
                xAxes: [{
                    type: 'time',
                    distribution: 'series',
                    time: {
                        // parser: timeFormat,
                        round: 'minute',
                        tooltipFormat: 'YYYY-MM-DD ll HH:mm',
                        displayFormats: {
                            'millisecond': 'MMM DD HH:mm',
                            'second': 'MMM DD HH:mm',
                            'minute': 'MMM DD HH:mm',
                            'hour': 'MMM DD HH:mm',
                            'day': 'MMM DD HH:mm',
                            'week': 'MMM DD HH:mm',
                            'month': 'MMM DD HH:mm',
                            'quarter': 'MMM DD HH:mm',
                            'year': 'MMM DD HH:mm',
                        },

                    },
                    offset: true,
                    scaleLabel: {
                        display: true,
                        labelString: 'Date'
                    },
                    ticks: {
                        major: {
                            enabled: true,
                            fontStyle: 'bold'
                        },
                        source: 'data',
                        autoSkip: true,
                        autoSkipPadding: 75,
                        maxRotation: 0,
                        sampleSize: 100
                    }
                }],
                yAxes: [{
                    gridLines: {
                        drawBorder: false
                    },
                    ticks: {
                        suggestedMin: 0,
                        suggestedMax: 100
                    },
                    scaleLabel: {
                        display: true,
                        labelString: 'Soil Humidity  (%)'
                    }
                }]
            }
        }
    });
    return "";
}