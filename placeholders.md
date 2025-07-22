# What is this?
These are the documented placeholders you can use in your messages (in the config).

Placeholders are always surrounded by `{}`, so if you want to use the `channel_name` placeholder, you'd type `{channel_name}`

# Listeners

## Global
- `channel_name`: The name of the twitch channel the bot entered.

### `eidolon_hunts`
Empty  

### `s_tier_arbitrations`
- `node`: The Node the arbitration is happening on
- `planet`: The Planet the node belongs to

### `meta_relics`
- `node`: The node AND planet, in the following format: `Node (Planet)`
- `difficulty`: A string indicating the difficulty. This is either `Normal` or `Steel Path`

### `steel_path_disruption_fissures`
- `node`: The node AND planet, in the following format: `Node (Planet)`

# Commands
## Global
- `author`: The person who sent the command.

### `!avg`
- `average`: The item's average price
- `moving_average`: The item's moving average price
- `item_name`: The queried item's CORRECTED name
- `amount_sold`: The amount of sold items in the last 48hrs

#### Filters
Filters can be applied via `||` as separator. For example: `!avg primed pressure p || r10`

- `r<number>`: filter by mod rank, if present on an item
