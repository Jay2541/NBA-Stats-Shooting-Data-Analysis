# NBA Stats Shooting Data Analysis
 
The following is a quick overview of how to navigate through this project folder:

Inside of the "NBA Stats (1947-Present)" folder, all of the CSV files from the dataset are present. In this project, only "Player Shooting.csv" and "Team Stats Per Game" are utilized.

Inside of the "src" folder, there are six Rust files of code.

- "analytics.rs" is in charge of creating the ratio and differences for each player's shooting statistics versus their team's respective statistics.
- "centrality.rs" is in charge of calculating the betweenness and closeness centrality for the nodes (for a description of the nodes, check "Jay Patel - DS210 Final Project Write-Up".
- "data_loader.rs" is in charge of loading the player and team data.
- "data_structures.rs" is in charge of creating structures that the Player, Team, and MergedData objects can follow.
- "graph.rs" is in charge of creating the graph using "PetGraph" for the nodes and edges.
- "main.rs" includes a sum up of all of the functions from the previous modules (whichever ones are necessary), as well as some additional functions to access and utilize the CSV files and create the new CSV files.

"analytics.rs", "centrality.rs", "graph.rs", and "main.rs" all include tests.

The final write-up is included in "Jay Patel - DS210 Final Project Write-Up".

The output CSV files are named, "Centrality Scores.csv", "Player Shooting Stats Analytics.csv", and "Players' Contribution To Team.csv".

- "Centrality Scores.csv" outputs the node and player that the node represents, along with the betweenness and closeness centrality methods which analyze how connected a player is in terms of their statistics and impact to the rest of their team. The results may vary depending on the play time of the player or the location on the graph made for that player relative to the rest of their team.
- "Player Shooting Stats Analytics.csv" takes a more statistical approach, outputting the player and team they are on. With that, a statistic is presented, with the "correlation coefficient," which in this case just represents the coefficient for how the player is correlated to the team's average. Shooting analytics are looked at to see how effective a player is from a specific range of shooting versus the rest of their team.
- "Players' Contribution To Team.csv" outputs statistics relative to how they impact a team's playoff success. All players are looked at in this area of focus, in order to see which area a player must excel in the most in order to help their team reach the playoffs.

Finally, the "Cargo.toml" file includes all of the directories necessary for the project to functionally run correctly.

This program filters and looks at players from the 2022 season.

Works Cited (code citations):

- ChatGPT (OpenAI)
- URL: chat.openai.com
