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

Finally, the "Cargo.toml" file includes all of the directories necessary for the project to functionally run correctly.
