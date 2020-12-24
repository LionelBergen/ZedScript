# ZedScript
League Of Legends API Wrapper for Rust - very easy to use


Every method from the Riot API are wrapped inside easy to use functions. Below are just a few examples to help you get started.

<details><summary>Example get_api_status - returns a 'SharedStatus', containing status info:</summary>

<p>

####

```rust
    let x : LolApiKey = LolApiKey {api_key: env::var("LEAGUE_API_KEY").unwrap().to_string(), region: Region::NA };
    let result = RiotApi::get_status(&x).unwrap();

    println!("{:?}", result);
```
</p>
</details>

<details><summary>Example get_champion_mastery - returns a list of 'ChampionMasteryDto', containing all the champion masteries for the summonerId passed:</summary>

<p>

####

```rust
    let api_key : LolApiKey = LolApiKey {api_key: env::var("LEAGUE_API_KEY").unwrap().to_string(), region: Region::NA };
	// LeagueOfSausage SummonerId
    let result = RiotApi::get_champion_mastery(&api_key, &"n-zcEtpy2E4JUt8AksUMpkEB9SsBw51-6b6rDF27wvZ1YYw".to_string());

    println!("{:?}", result);
```
</p>
</details>



# Disclaimer
`ZedScript` is **not** endorsed by Riot Games and does **not** reflect the views or opinions of Riot Games or anyone officially involved in producing or managing League of Legends. League of Legends and Riot Games are trademarks or registered trademarks of Riot Games, Inc. League of Legends © Riot Games, Inc.

