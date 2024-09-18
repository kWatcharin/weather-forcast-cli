# Weather Forcasting CLI

## Feature
![image](https://github.com/kWatcharin/weather-forcast-cli/issues/1#issue-2533658566.jpg)

## Installation.
```shell
git clone https://github.com/kWatcharin/weather-forcast-cli.git
```

## Get API key.
### Go to OpenWeather and register.
See other detail about [OpenWeather](https://openweathermap.org/):

![image](https://github.com/kWatcharin/weather-forcast-cli/issues/2#issue-2533694134.jpg)

## Setup env file.
```shell
cd weather-forcast-cli
# Create .env file
```
Add your api key from openweather to .env file 
```env
API_KEY = myapikey12345 # Your API key
```

### Compile
```shell
cargo build --release
```

### Run Application
```shell
.\target\release\weather-forcast-cli
```