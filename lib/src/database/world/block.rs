macro_rules! block_generator {
    (
        $(
            $item:ident => $tag:ident
        )*
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Block {
            $(
                $item,
            )*
        }

        impl Block {
            pub fn tag(&self) -> &'static str {
                match self {
                    $(
                        Block::$item => stringify!($tag),
                    )*
                }
            }

            pub fn from_tag(tag: &str) -> Option<Block> {
                match tag {
                    $(
                        stringify!($tag) => Some(Block::$item),
                    )*
                    _ => None,
                }
            }
        }

        impl serde::Serialize for Block {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.tag())
            }
        }

        impl<'de> serde::Deserialize<'de> for Block {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> {
                Block::from_tag(&String::deserialize(deserializer)?).ok_or_else(|| serde::de::Error::custom("unknown block tag"))
            }
        }
    };
}

block_generator!(
    Air => air
    Stone => stone
    Granite => granite
    PolishedGranite => polished_granite
    Diorite => diorite
    PolishedDiorite => polished_diorite
    Andesite => andesite
    PolishedAndesite => polished_andesite
    GrassBlock => grass_block
    Dirt => dirt
    CoarseDirt => coarse_dirt
    Podzol => podzol
    Cobblestone => cobblestone
    OakPlanks => oak_planks
    SprucePlanks => spruce_planks
    BirchPlanks => birch_planks
    JunglePlanks => jungle_planks
    AcaciaPlanks => acacia_planks
    CherryPlanks => cherry_planks
    DarkOakPlanks => dark_oak_planks
    MangrovePlanks => mangrove_planks
    BambooPlanks => bamboo_planks
    BambooMosaic => bamboo_mosaic
    OakSapling => oak_sapling
    SpruceSapling => spruce_sapling
    BirchSapling => birch_sapling
    JungleSapling => jungle_sapling
    AcaciaSapling => acacia_sapling
    CherrySapling => cherry_sapling
    DarkOakSapling => dark_oak_sapling
    MangrovePropagule => mangrove_propagule
    Bedrock => bedrock
    Water => water
    Lava => lava
    Sand => sand
    SuspiciousSand => suspicious_sand
    RedSand => red_sand
    Gravel => gravel
    SuspiciousGravel => suspicious_gravel
    GoldOre => gold_ore
    DeepslateGoldOre => deepslate_gold_ore
    IronOre => iron_ore
    DeepslateIronOre => deepslate_iron_ore
    CoalOre => coal_ore
    DeepslateCoalOre => deepslate_coal_ore
    NetherGoldOre => nether_gold_ore
    OakLog => oak_log
    SpruceLog => spruce_log
    BirchLog => birch_log
    JungleLog => jungle_log
    AcaciaLog => acacia_log
    CherryLog => cherry_log
    DarkOakLog => dark_oak_log
    MangroveLog => mangrove_log
    MangroveRoots => mangrove_roots
    MuddyMangroveRoots => muddy_mangrove_roots
    BlockofBamboo => bamboo_block
    StrippedSpruceLog => stripped_spruce_log
    StrippedBirchLog => stripped_birch_log
    StrippedJungleLog => stripped_jungle_log
    StrippedAcaciaLog => stripped_acacia_log
    StrippedCherryLog => stripped_cherry_log
    StrippedDarkOakLog => stripped_dark_oak_log
    StrippedOakLog => stripped_oak_log
    StrippedMangroveLog => stripped_mangrove_log
    BlockofStrippedBamboo => stripped_bamboo_block
    OakWood => oak_wood
    SpruceWood => spruce_wood
    BirchWood => birch_wood
    JungleWood => jungle_wood
    AcaciaWood => acacia_wood
    CherryWood => cherry_wood
    DarkOakWood => dark_oak_wood
    MangroveWood => mangrove_wood
    StrippedOakWood => stripped_oak_wood
    StrippedSpruceWood => stripped_spruce_wood
    StrippedBirchWood => stripped_birch_wood
    StrippedJungleWood => stripped_jungle_wood
    StrippedAcaciaWood => stripped_acacia_wood
    StrippedCherryWood => stripped_cherry_wood
    StrippedDarkOakWood => stripped_dark_oak_wood
    StrippedMangroveWood => stripped_mangrove_wood
    OakLeaves => oak_leaves
    SpruceLeaves => spruce_leaves
    BirchLeaves => birch_leaves
    JungleLeaves => jungle_leaves
    AcaciaLeaves => acacia_leaves
    CherryLeaves => cherry_leaves
    DarkOakLeaves => dark_oak_leaves
    MangroveLeaves => mangrove_leaves
    AzaleaLeaves => azalea_leaves
    FloweringAzaleaLeaves => flowering_azalea_leaves
    Sponge => sponge
    WetSponge => wet_sponge
    Glass => glass
    LapisLazuliOre => lapis_ore
    DeepslateLapisLazuliOre => deepslate_lapis_ore
    BlockofLapisLazuli => lapis_block
    Dispenser => dispenser
    Sandstone => sandstone
    ChiseledSandstone => chiseled_sandstone
    CutSandstone => cut_sandstone
    NoteBlock => note_block
    WhiteBed => white_bed
    OrangeBed => orange_bed
    MagentaBed => magenta_bed
    LightBlueBed => light_blue_bed
    YellowBed => yellow_bed
    LimeBed => lime_bed
    PinkBed => pink_bed
    GrayBed => gray_bed
    LightGrayBed => light_gray_bed
    CyanBed => cyan_bed
    PurpleBed => purple_bed
    BlueBed => blue_bed
    BrownBed => brown_bed
    GreenBed => green_bed
    RedBed => red_bed
    BlackBed => black_bed
    PoweredRail => powered_rail
    DetectorRail => detector_rail
    StickyPiston => sticky_piston
    Cobweb => cobweb
    Grass => grass
    Fern => fern
    DeadBush => dead_bush
    Seagrass => seagrass
    TallSeagrass => tall_seagrass
    Piston => piston
    PistonHead => piston_head
    WhiteWool => white_wool
    OrangeWool => orange_wool
    MagentaWool => magenta_wool
    LightBlueWool => light_blue_wool
    YellowWool => yellow_wool
    LimeWool => lime_wool
    PinkWool => pink_wool
    GrayWool => gray_wool
    LightGrayWool => light_gray_wool
    CyanWool => cyan_wool
    PurpleWool => purple_wool
    BlueWool => blue_wool
    BrownWool => brown_wool
    GreenWool => green_wool
    RedWool => red_wool
    BlackWool => black_wool
    MovingPiston => moving_piston
    Dandelion => dandelion
    Torchflower => torchflower
    Poppy => poppy
    BlueOrchid => blue_orchid
    Allium => allium
    AzureBluet => azure_bluet
    RedTulip => red_tulip
    OrangeTulip => orange_tulip
    WhiteTulip => white_tulip
    PinkTulip => pink_tulip
    OxeyeDaisy => oxeye_daisy
    Cornflower => cornflower
    WitherRose => wither_rose
    LilyoftheValley => lily_of_the_valley
    BrownMushroom => brown_mushroom
    RedMushroom => red_mushroom
    BlockofGold => gold_block
    BlockofIron => iron_block
    Bricks => bricks
    TNT => tnt
    Bookshelf => bookshelf
    ChiseledBookshelf => chiseled_bookshelf
    MossyCobblestone => mossy_cobblestone
    Obsidian => obsidian
    Torch => torch
    WallTorch => wall_torch
    Fire => fire
    SoulFire => soul_fire
    MonsterSpawner => spawner
    OakStairs => oak_stairs
    Chest => chest
    RedstoneWire => redstone_wire
    DiamondOre => diamond_ore
    DeepslateDiamondOre => deepslate_diamond_ore
    BlockofDiamond => diamond_block
    CraftingTable => crafting_table
    WheatCrops => wheat
    Farmland => farmland
    Furnace => furnace
    OakSign => oak_sign
    SpruceSign => spruce_sign
    BirchSign => birch_sign
    AcaciaSign => acacia_sign
    CherrySign => cherry_sign
    JungleSign => jungle_sign
    DarkOakSign => dark_oak_sign
    MangroveSign => mangrove_sign
    BambooSign => bamboo_sign
    OakDoor => oak_door
    Ladder => ladder
    Rail => rail
    CobblestoneStairs => cobblestone_stairs
    OakWallSign => oak_wall_sign
    SpruceWallSign => spruce_wall_sign
    BirchWallSign => birch_wall_sign
    AcaciaWallSign => acacia_wall_sign
    CherryWallSign => cherry_wall_sign
    JungleWallSign => jungle_wall_sign
    DarkOakWallSign => dark_oak_wall_sign
    MangroveWallSign => mangrove_wall_sign
    BambooWallSign => bamboo_wall_sign
    OakHangingSign => oak_hanging_sign
    SpruceHangingSign => spruce_hanging_sign
    BirchHangingSign => birch_hanging_sign
    AcaciaHangingSign => acacia_hanging_sign
    CherryHangingSign => cherry_hanging_sign
    JungleHangingSign => jungle_hanging_sign
    DarkOakHangingSign => dark_oak_hanging_sign
    CrimsonHangingSign => crimson_hanging_sign
    WarpedHangingSign => warped_hanging_sign
    MangroveHangingSign => mangrove_hanging_sign
    BambooHangingSign => bamboo_hanging_sign
    OakWallHangingSign => oak_wall_hanging_sign
    SpruceWallHangingSign => spruce_wall_hanging_sign
    BirchWallHangingSign => birch_wall_hanging_sign
    AcaciaWallHangingSign => acacia_wall_hanging_sign
    CherryWallHangingSign => cherry_wall_hanging_sign
    JungleWallHangingSign => jungle_wall_hanging_sign
    DarkOakWallHangingSign => dark_oak_wall_hanging_sign
    MangroveWallHangingSign => mangrove_wall_hanging_sign
    CrimsonWallHangingSign => crimson_wall_hanging_sign
    WarpedWallHangingSign => warped_wall_hanging_sign
    BambooWallHangingSign => bamboo_wall_hanging_sign
    Lever => lever
    StonePressurePlate => stone_pressure_plate
    IronDoor => iron_door
    OakPressurePlate => oak_pressure_plate
    SprucePressurePlate => spruce_pressure_plate
    BirchPressurePlate => birch_pressure_plate
    JunglePressurePlate => jungle_pressure_plate
    AcaciaPressurePlate => acacia_pressure_plate
    CherryPressurePlate => cherry_pressure_plate
    DarkOakPressurePlate => dark_oak_pressure_plate
    MangrovePressurePlate => mangrove_pressure_plate
    BambooPressurePlate => bamboo_pressure_plate
    RedstoneOre => redstone_ore
    DeepslateRedstoneOre => deepslate_redstone_ore
    RedstoneTorch => redstone_torch
    RedstoneWallTorch => redstone_wall_torch
    StoneButton => stone_button
    Snow => snow
    Ice => ice
    SnowBlock => snow_block
    Cactus => cactus
    Clay => clay
    SugarCane => sugar_cane
    Jukebox => jukebox
    OakFence => oak_fence
    Pumpkin => pumpkin
    Netherrack => netherrack
    SoulSand => soul_sand
    SoulSoil => soul_soil
    Basalt => basalt
    PolishedBasalt => polished_basalt
    SoulTorch => soul_torch
    SoulWallTorch => soul_wall_torch
    Glowstone => glowstone
    NetherPortal => nether_portal
    CarvedPumpkin => carved_pumpkin
    JackoLantern => jack_o_lantern
    Cake => cake
    RedstoneRepeater => repeater
    WhiteStainedGlass => white_stained_glass
    OrangeStainedGlass => orange_stained_glass
    MagentaStainedGlass => magenta_stained_glass
    LightBlueStainedGlass => light_blue_stained_glass
    YellowStainedGlass => yellow_stained_glass
    LimeStainedGlass => lime_stained_glass
    PinkStainedGlass => pink_stained_glass
    GrayStainedGlass => gray_stained_glass
    LightGrayStainedGlass => light_gray_stained_glass
    CyanStainedGlass => cyan_stained_glass
    PurpleStainedGlass => purple_stained_glass
    BlueStainedGlass => blue_stained_glass
    BrownStainedGlass => brown_stained_glass
    GreenStainedGlass => green_stained_glass
    RedStainedGlass => red_stained_glass
    BlackStainedGlass => black_stained_glass
    OakTrapdoor => oak_trapdoor
    SpruceTrapdoor => spruce_trapdoor
    BirchTrapdoor => birch_trapdoor
    JungleTrapdoor => jungle_trapdoor
    AcaciaTrapdoor => acacia_trapdoor
    CherryTrapdoor => cherry_trapdoor
    DarkOakTrapdoor => dark_oak_trapdoor
    MangroveTrapdoor => mangrove_trapdoor
    BambooTrapdoor => bamboo_trapdoor
    StoneBricks => stone_bricks
    MossyStoneBricks => mossy_stone_bricks
    CrackedStoneBricks => cracked_stone_bricks
    ChiseledStoneBricks => chiseled_stone_bricks
    PackedMud => packed_mud
    MudBricks => mud_bricks
    InfestedStone => infested_stone
    InfestedCobblestone => infested_cobblestone
    InfestedStoneBricks => infested_stone_bricks
    InfestedMossyStoneBricks => infested_mossy_stone_bricks
    InfestedCrackedStoneBricks => infested_cracked_stone_bricks
    InfestedChiseledStoneBricks => infested_chiseled_stone_bricks
    BrownMushroomBlock => brown_mushroom_block
    RedMushroomBlock => red_mushroom_block
    MushroomStem => mushroom_stem
    IronBars => iron_bars
    Chain => chain
    GlassPane => glass_pane
    Melon => melon
    AttachedPumpkinStem => attached_pumpkin_stem
    AttachedMelonStem => attached_melon_stem
    PumpkinStem => pumpkin_stem
    MelonStem => melon_stem
    Vines => vine
    GlowLichen => glow_lichen
    OakFenceGate => oak_fence_gate
    BrickStairs => brick_stairs
    StoneBrickStairs => stone_brick_stairs
    MudBrickStairs => mud_brick_stairs
    Mycelium => mycelium
    LilyPad => lily_pad
    NetherBricks => nether_bricks
    NetherBrickFence => nether_brick_fence
    NetherBrickStairs => nether_brick_stairs
    NetherWart => nether_wart
    EnchantingTable => enchanting_table
    BrewingStand => brewing_stand
    Cauldron => cauldron
    WaterCauldron => water_cauldron
    LavaCauldron => lava_cauldron
    PowderSnowCauldron => powder_snow_cauldron
    EndPortal => end_portal
    EndPortalFrame => end_portal_frame
    EndStone => end_stone
    DragonEgg => dragon_egg
    RedstoneLamp => redstone_lamp
    Cocoa => cocoa
    SandstoneStairs => sandstone_stairs
    EmeraldOre => emerald_ore
    DeepslateEmeraldOre => deepslate_emerald_ore
    EnderChest => ender_chest
    TripwireHook => tripwire_hook
    Tripwire => tripwire
    BlockofEmerald => emerald_block
    SpruceStairs => spruce_stairs
    BirchStairs => birch_stairs
    JungleStairs => jungle_stairs
    CommandBlock => command_block
    Beacon => beacon
    CobblestoneWall => cobblestone_wall
    MossyCobblestoneWall => mossy_cobblestone_wall
    FlowerPot => flower_pot
    PottedTorchflower => potted_torchflower
    PottedOakSapling => potted_oak_sapling
    PottedSpruceSapling => potted_spruce_sapling
    PottedBirchSapling => potted_birch_sapling
    PottedJungleSapling => potted_jungle_sapling
    PottedAcaciaSapling => potted_acacia_sapling
    PottedCherrySapling => potted_cherry_sapling
    PottedDarkOakSapling => potted_dark_oak_sapling
    PottedMangrovePropagule => potted_mangrove_propagule
    PottedFern => potted_fern
    PottedDandelion => potted_dandelion
    PottedPoppy => potted_poppy
    PottedBlueOrchid => potted_blue_orchid
    PottedAllium => potted_allium
    PottedAzureBluet => potted_azure_bluet
    PottedRedTulip => potted_red_tulip
    PottedOrangeTulip => potted_orange_tulip
    PottedWhiteTulip => potted_white_tulip
    PottedPinkTulip => potted_pink_tulip
    PottedOxeyeDaisy => potted_oxeye_daisy
    PottedCornflower => potted_cornflower
    PottedLilyoftheValley => potted_lily_of_the_valley
    PottedWitherRose => potted_wither_rose
    PottedRedMushroom => potted_red_mushroom
    PottedBrownMushroom => potted_brown_mushroom
    PottedDeadBush => potted_dead_bush
    PottedCactus => potted_cactus
    Carrots => carrots
    Potatoes => potatoes
    OakButton => oak_button
    SpruceButton => spruce_button
    BirchButton => birch_button
    JungleButton => jungle_button
    AcaciaButton => acacia_button
    CherryButton => cherry_button
    DarkOakButton => dark_oak_button
    MangroveButton => mangrove_button
    BambooButton => bamboo_button
    SkeletonSkull => skeleton_skull
    SkeletonWallSkull => skeleton_wall_skull
    WitherSkeletonSkull => wither_skeleton_skull
    WitherSkeletonWallSkull => wither_skeleton_wall_skull
    ZombieHead => zombie_head
    ZombieWallHead => zombie_wall_head
    PlayerHead => player_head
    PlayerWallHead => player_wall_head
    CreeperHead => creeper_head
    CreeperWallHead => creeper_wall_head
    DragonHead => dragon_head
    DragonWallHead => dragon_wall_head
    PiglinHead => piglin_head
    PiglinWallHead => piglin_wall_head
    Anvil => anvil
    ChippedAnvil => chipped_anvil
    DamagedAnvil => damaged_anvil
    TrappedChest => trapped_chest
    LightWeightedPressurePlate => light_weighted_pressure_plate
    HeavyWeightedPressurePlate => heavy_weighted_pressure_plate
    RedstoneComparator => comparator
    DaylightDetector => daylight_detector
    BlockofRedstone => redstone_block
    NetherQuartzOre => nether_quartz_ore
    Hopper => hopper
    BlockofQuartz => quartz_block
    ChiseledQuartzBlock => chiseled_quartz_block
    QuartzPillar => quartz_pillar
    QuartzStairs => quartz_stairs
    ActivatorRail => activator_rail
    Dropper => dropper
    WhiteTerracotta => white_terracotta
    OrangeTerracotta => orange_terracotta
    MagentaTerracotta => magenta_terracotta
    LightBlueTerracotta => light_blue_terracotta
    YellowTerracotta => yellow_terracotta
    LimeTerracotta => lime_terracotta
    PinkTerracotta => pink_terracotta
    GrayTerracotta => gray_terracotta
    LightGrayTerracotta => light_gray_terracotta
    CyanTerracotta => cyan_terracotta
    PurpleTerracotta => purple_terracotta
    BlueTerracotta => blue_terracotta
    BrownTerracotta => brown_terracotta
    GreenTerracotta => green_terracotta
    RedTerracotta => red_terracotta
    BlackTerracotta => black_terracotta
    WhiteStainedGlassPane => white_stained_glass_pane
    OrangeStainedGlassPane => orange_stained_glass_pane
    MagentaStainedGlassPane => magenta_stained_glass_pane
    LightBlueStainedGlassPane => light_blue_stained_glass_pane
    YellowStainedGlassPane => yellow_stained_glass_pane
    LimeStainedGlassPane => lime_stained_glass_pane
    PinkStainedGlassPane => pink_stained_glass_pane
    GrayStainedGlassPane => gray_stained_glass_pane
    LightGrayStainedGlassPane => light_gray_stained_glass_pane
    CyanStainedGlassPane => cyan_stained_glass_pane
    PurpleStainedGlassPane => purple_stained_glass_pane
    BlueStainedGlassPane => blue_stained_glass_pane
    BrownStainedGlassPane => brown_stained_glass_pane
    GreenStainedGlassPane => green_stained_glass_pane
    RedStainedGlassPane => red_stained_glass_pane
    BlackStainedGlassPane => black_stained_glass_pane
    AcaciaStairs => acacia_stairs
    CherryStairs => cherry_stairs
    DarkOakStairs => dark_oak_stairs
    MangroveStairs => mangrove_stairs
    BambooStairs => bamboo_stairs
    BambooMosaicStairs => bamboo_mosaic_stairs
    SlimeBlock => slime_block
    Barrier => barrier
    Light => light
    IronTrapdoor => iron_trapdoor
    Prismarine => prismarine
    PrismarineBricks => prismarine_bricks
    DarkPrismarine => dark_prismarine
    PrismarineStairs => prismarine_stairs
    PrismarineBrickStairs => prismarine_brick_stairs
    DarkPrismarineStairs => dark_prismarine_stairs
    PrismarineSlab => prismarine_slab
    PrismarineBrickSlab => prismarine_brick_slab
    DarkPrismarineSlab => dark_prismarine_slab
    SeaLantern => sea_lantern
    HayBale => hay_block
    WhiteCarpet => white_carpet
    OrangeCarpet => orange_carpet
    MagentaCarpet => magenta_carpet
    LightBlueCarpet => light_blue_carpet
    YellowCarpet => yellow_carpet
    LimeCarpet => lime_carpet
    PinkCarpet => pink_carpet
    GrayCarpet => gray_carpet
    LightGrayCarpet => light_gray_carpet
    CyanCarpet => cyan_carpet
    PurpleCarpet => purple_carpet
    BlueCarpet => blue_carpet
    BrownCarpet => brown_carpet
    GreenCarpet => green_carpet
    RedCarpet => red_carpet
    BlackCarpet => black_carpet
    Terracotta => terracotta
    BlockofCoal => coal_block
    PackedIce => packed_ice
    Sunflower => sunflower
    Lilac => lilac
    RoseBush => rose_bush
    Peony => peony
    TallGrass => tall_grass
    LargeFern => large_fern
    WhiteBanner => white_banner
    OrangeBanner => orange_banner
    MagentaBanner => magenta_banner
    LightBlueBanner => light_blue_banner
    YellowBanner => yellow_banner
    LimeBanner => lime_banner
    PinkBanner => pink_banner
    GrayBanner => gray_banner
    LightGrayBanner => light_gray_banner
    CyanBanner => cyan_banner
    PurpleBanner => purple_banner
    BlueBanner => blue_banner
    BrownBanner => brown_banner
    GreenBanner => green_banner
    RedBanner => red_banner
    BlackBanner => black_banner
    Whitewallbanner => white_wall_banner
    Orangewallbanner => orange_wall_banner
    Magentawallbanner => magenta_wall_banner
    Lightbluewallbanner => light_blue_wall_banner
    Yellowwallbanner => yellow_wall_banner
    Limewallbanner => lime_wall_banner
    Pinkwallbanner => pink_wall_banner
    Graywallbanner => gray_wall_banner
    Lightgraywallbanner => light_gray_wall_banner
    Cyanwallbanner => cyan_wall_banner
    Purplewallbanner => purple_wall_banner
    Bluewallbanner => blue_wall_banner
    Brownwallbanner => brown_wall_banner
    Greenwallbanner => green_wall_banner
    Redwallbanner => red_wall_banner
    Blackwallbanner => black_wall_banner
    RedSandstone => red_sandstone
    ChiseledRedSandstone => chiseled_red_sandstone
    CutRedSandstone => cut_red_sandstone
    RedSandstoneStairs => red_sandstone_stairs
    OakSlab => oak_slab
    SpruceSlab => spruce_slab
    BirchSlab => birch_slab
    JungleSlab => jungle_slab
    AcaciaSlab => acacia_slab
    CherrySlab => cherry_slab
    DarkOakSlab => dark_oak_slab
    MangroveSlab => mangrove_slab
    BambooSlab => bamboo_slab
    BambooMosaicSlab => bamboo_mosaic_slab
    StoneSlab => stone_slab
    SmoothStoneSlab => smooth_stone_slab
    SandstoneSlab => sandstone_slab
    CutSandstoneSlab => cut_sandstone_slab
    PetrifiedOakSlab => petrified_oak_slab
    CobblestoneSlab => cobblestone_slab
    BrickSlab => brick_slab
    StoneBrickSlab => stone_brick_slab
    MudBrickSlab => mud_brick_slab
    NetherBrickSlab => nether_brick_slab
    QuartzSlab => quartz_slab
    RedSandstoneSlab => red_sandstone_slab
    CutRedSandstoneSlab => cut_red_sandstone_slab
    PurpurSlab => purpur_slab
    SmoothStone => smooth_stone
    SmoothSandstone => smooth_sandstone
    SmoothQuartzBlock => smooth_quartz
    SmoothRedSandstone => smooth_red_sandstone
    SpruceFenceGate => spruce_fence_gate
    BirchFenceGate => birch_fence_gate
    JungleFenceGate => jungle_fence_gate
    AcaciaFenceGate => acacia_fence_gate
    CherryFenceGate => cherry_fence_gate
    DarkOakFenceGate => dark_oak_fence_gate
    MangroveFenceGate => mangrove_fence_gate
    BambooFenceGate => bamboo_fence_gate
    SpruceFence => spruce_fence
    BirchFence => birch_fence
    JungleFence => jungle_fence
    AcaciaFence => acacia_fence
    CherryFence => cherry_fence
    DarkOakFence => dark_oak_fence
    MangroveFence => mangrove_fence
    BambooFence => bamboo_fence
    SpruceDoor => spruce_door
    BirchDoor => birch_door
    JungleDoor => jungle_door
    AcaciaDoor => acacia_door
    CherryDoor => cherry_door
    DarkOakDoor => dark_oak_door
    MangroveDoor => mangrove_door
    BambooDoor => bamboo_door
    EndRod => end_rod
    ChorusPlant => chorus_plant
    ChorusFlower => chorus_flower
    PurpurBlock => purpur_block
    PurpurPillar => purpur_pillar
    PurpurStairs => purpur_stairs
    EndStoneBricks => end_stone_bricks
    TorchflowerCrop => torchflower_crop
    PitcherCrop => pitcher_crop
    PitcherPlant => pitcher_plant
    Beetroots => beetroots
    DirtPath => dirt_path
    EndGateway => end_gateway
    RepeatingCommandBlock => repeating_command_block
    ChainCommandBlock => chain_command_block
    FrostedIce => frosted_ice
    MagmaBlock => magma_block
    NetherWartBlock => nether_wart_block
    RedNetherBricks => red_nether_bricks
    BoneBlock => bone_block
    StructureVoid => structure_void
    Observer => observer
    ShulkerBox => shulker_box
    WhiteShulkerBox => white_shulker_box
    OrangeShulkerBox => orange_shulker_box
    MagentaShulkerBox => magenta_shulker_box
    LightBlueShulkerBox => light_blue_shulker_box
    YellowShulkerBox => yellow_shulker_box
    LimeShulkerBox => lime_shulker_box
    PinkShulkerBox => pink_shulker_box
    GrayShulkerBox => gray_shulker_box
    LightGrayShulkerBox => light_gray_shulker_box
    CyanShulkerBox => cyan_shulker_box
    PurpleShulkerBox => purple_shulker_box
    BlueShulkerBox => blue_shulker_box
    BrownShulkerBox => brown_shulker_box
    GreenShulkerBox => green_shulker_box
    RedShulkerBox => red_shulker_box
    BlackShulkerBox => black_shulker_box
    WhiteGlazedTerracotta => white_glazed_terracotta
    OrangeGlazedTerracotta => orange_glazed_terracotta
    MagentaGlazedTerracotta => magenta_glazed_terracotta
    LightBlueGlazedTerracotta => light_blue_glazed_terracotta
    YellowGlazedTerracotta => yellow_glazed_terracotta
    LimeGlazedTerracotta => lime_glazed_terracotta
    PinkGlazedTerracotta => pink_glazed_terracotta
    GrayGlazedTerracotta => gray_glazed_terracotta
    LightGrayGlazedTerracotta => light_gray_glazed_terracotta
    CyanGlazedTerracotta => cyan_glazed_terracotta
    PurpleGlazedTerracotta => purple_glazed_terracotta
    BlueGlazedTerracotta => blue_glazed_terracotta
    BrownGlazedTerracotta => brown_glazed_terracotta
    GreenGlazedTerracotta => green_glazed_terracotta
    RedGlazedTerracotta => red_glazed_terracotta
    BlackGlazedTerracotta => black_glazed_terracotta
    WhiteConcrete => white_concrete
    OrangeConcrete => orange_concrete
    MagentaConcrete => magenta_concrete
    LightBlueConcrete => light_blue_concrete
    YellowConcrete => yellow_concrete
    LimeConcrete => lime_concrete
    PinkConcrete => pink_concrete
    GrayConcrete => gray_concrete
    LightGrayConcrete => light_gray_concrete
    CyanConcrete => cyan_concrete
    PurpleConcrete => purple_concrete
    BlueConcrete => blue_concrete
    BrownConcrete => brown_concrete
    GreenConcrete => green_concrete
    RedConcrete => red_concrete
    BlackConcrete => black_concrete
    WhiteConcretePowder => white_concrete_powder
    OrangeConcretePowder => orange_concrete_powder
    MagentaConcretePowder => magenta_concrete_powder
    LightBlueConcretePowder => light_blue_concrete_powder
    YellowConcretePowder => yellow_concrete_powder
    LimeConcretePowder => lime_concrete_powder
    PinkConcretePowder => pink_concrete_powder
    GrayConcretePowder => gray_concrete_powder
    LightGrayConcretePowder => light_gray_concrete_powder
    CyanConcretePowder => cyan_concrete_powder
    PurpleConcretePowder => purple_concrete_powder
    BlueConcretePowder => blue_concrete_powder
    BrownConcretePowder => brown_concrete_powder
    GreenConcretePowder => green_concrete_powder
    RedConcretePowder => red_concrete_powder
    BlackConcretePowder => black_concrete_powder
    Kelp => kelp
    KelpPlant => kelp_plant
    DriedKelpBlock => dried_kelp_block
    TurtleEgg => turtle_egg
    SnifferEgg => sniffer_egg
    DeadTubeCoralBlock => dead_tube_coral_block
    DeadBrainCoralBlock => dead_brain_coral_block
    DeadBubbleCoralBlock => dead_bubble_coral_block
    DeadFireCoralBlock => dead_fire_coral_block
    DeadHornCoralBlock => dead_horn_coral_block
    TubeCoralBlock => tube_coral_block
    BrainCoralBlock => brain_coral_block
    BubbleCoralBlock => bubble_coral_block
    FireCoralBlock => fire_coral_block
    HornCoralBlock => horn_coral_block
    DeadTubeCoral => dead_tube_coral
    DeadBrainCoral => dead_brain_coral
    DeadBubbleCoral => dead_bubble_coral
    DeadFireCoral => dead_fire_coral
    DeadHornCoral => dead_horn_coral
    TubeCoral => tube_coral
    BrainCoral => brain_coral
    BubbleCoral => bubble_coral
    FireCoral => fire_coral
    HornCoral => horn_coral
    DeadTubeCoralFan => dead_tube_coral_fan
    DeadBrainCoralFan => dead_brain_coral_fan
    DeadBubbleCoralFan => dead_bubble_coral_fan
    DeadFireCoralFan => dead_fire_coral_fan
    DeadHornCoralFan => dead_horn_coral_fan
    TubeCoralFan => tube_coral_fan
    BrainCoralFan => brain_coral_fan
    BubbleCoralFan => bubble_coral_fan
    FireCoralFan => fire_coral_fan
    HornCoralFan => horn_coral_fan
    DeadTubeCoralWallFan => dead_tube_coral_wall_fan
    DeadBrainCoralWallFan => dead_brain_coral_wall_fan
    DeadBubbleCoralWallFan => dead_bubble_coral_wall_fan
    DeadFireCoralWallFan => dead_fire_coral_wall_fan
    DeadHornCoralWallFan => dead_horn_coral_wall_fan
    TubeCoralWallFan => tube_coral_wall_fan
    BrainCoralWallFan => brain_coral_wall_fan
    BubbleCoralWallFan => bubble_coral_wall_fan
    FireCoralWallFan => fire_coral_wall_fan
    HornCoralWallFan => horn_coral_wall_fan
    SeaPickle => sea_pickle
    BlueIce => blue_ice
    Conduit => conduit
    BambooShoot => bamboo_sapling
    Bamboo => bamboo
    PottedBamboo => potted_bamboo
    VoidAir => void_air
    CaveAir => cave_air
    BubbleColumn => bubble_column
    PolishedGraniteStairs => polished_granite_stairs
    SmoothRedSandstoneStairs => smooth_red_sandstone_stairs
    MossyStoneBrickStairs => mossy_stone_brick_stairs
    PolishedDioriteStairs => polished_diorite_stairs
    MossyCobblestoneStairs => mossy_cobblestone_stairs
    EndStoneBrickStairs => end_stone_brick_stairs
    StoneStairs => stone_stairs
    SmoothSandstoneStairs => smooth_sandstone_stairs
    SmoothQuartzStairs => smooth_quartz_stairs
    GraniteStairs => granite_stairs
    AndesiteStairs => andesite_stairs
    RedNetherBrickStairs => red_nether_brick_stairs
    PolishedAndesiteStairs => polished_andesite_stairs
    DioriteStairs => diorite_stairs
    PolishedGraniteSlab => polished_granite_slab
    SmoothRedSandstoneSlab => smooth_red_sandstone_slab
    MossyStoneBrickSlab => mossy_stone_brick_slab
    PolishedDioriteSlab => polished_diorite_slab
    MossyCobblestoneSlab => mossy_cobblestone_slab
    EndStoneBrickSlab => end_stone_brick_slab
    SmoothSandstoneSlab => smooth_sandstone_slab
    SmoothQuartzSlab => smooth_quartz_slab
    GraniteSlab => granite_slab
    AndesiteSlab => andesite_slab
    RedNetherBrickSlab => red_nether_brick_slab
    PolishedAndesiteSlab => polished_andesite_slab
    DioriteSlab => diorite_slab
    BrickWall => brick_wall
    PrismarineWall => prismarine_wall
    RedSandstoneWall => red_sandstone_wall
    MossyStoneBrickWall => mossy_stone_brick_wall
    GraniteWall => granite_wall
    StoneBrickWall => stone_brick_wall
    MudBrickWall => mud_brick_wall
    NetherBrickWall => nether_brick_wall
    AndesiteWall => andesite_wall
    RedNetherBrickWall => red_nether_brick_wall
    SandstoneWall => sandstone_wall
    EndStoneBrickWall => end_stone_brick_wall
    DioriteWall => diorite_wall
    Scaffolding => scaffolding
    Loom => loom
    Barrel => barrel
    Smoker => smoker
    BlastFurnace => blast_furnace
    CartographyTable => cartography_table
    FletchingTable => fletching_table
    Grindstone => grindstone
    Lectern => lectern
    SmithingTable => smithing_table
    Stonecutter => stonecutter
    Bell => bell
    Lantern => lantern
    SoulLantern => soul_lantern
    Campfire => campfire
    SoulCampfire => soul_campfire
    SweetBerryBush => sweet_berry_bush
    WarpedStem => warped_stem
    StrippedWarpedStem => stripped_warped_stem
    WarpedHyphae => warped_hyphae
    StrippedWarpedHyphae => stripped_warped_hyphae
    WarpedNylium => warped_nylium
    WarpedFungus => warped_fungus
    WarpedWartBlock => warped_wart_block
    WarpedRoots => warped_roots
    NetherSprouts => nether_sprouts
    CrimsonStem => crimson_stem
    StrippedCrimsonStem => stripped_crimson_stem
    CrimsonHyphae => crimson_hyphae
    StrippedCrimsonHyphae => stripped_crimson_hyphae
    CrimsonNylium => crimson_nylium
    CrimsonFungus => crimson_fungus
    Shroomlight => shroomlight
    WeepingVines => weeping_vines
    WeepingVinesPlant => weeping_vines_plant
    TwistingVines => twisting_vines
    TwistingVinesPlant => twisting_vines_plant
    CrimsonRoots => crimson_roots
    CrimsonPlanks => crimson_planks
    WarpedPlanks => warped_planks
    CrimsonSlab => crimson_slab
    WarpedSlab => warped_slab
    CrimsonPressurePlate => crimson_pressure_plate
    WarpedPressurePlate => warped_pressure_plate
    CrimsonFence => crimson_fence
    WarpedFence => warped_fence
    CrimsonTrapdoor => crimson_trapdoor
    WarpedTrapdoor => warped_trapdoor
    CrimsonFenceGate => crimson_fence_gate
    WarpedFenceGate => warped_fence_gate
    CrimsonStairs => crimson_stairs
    WarpedStairs => warped_stairs
    CrimsonButton => crimson_button
    WarpedButton => warped_button
    CrimsonDoor => crimson_door
    WarpedDoor => warped_door
    CrimsonSign => crimson_sign
    WarpedSign => warped_sign
    CrimsonWallSign => crimson_wall_sign
    WarpedWallSign => warped_wall_sign
    StructureBlock => structure_block
    JigsawBlock => jigsaw
    Composter => composter
    Target => target
    BeeNest => bee_nest
    Beehive => beehive
    HoneyBlock => honey_block
    HoneycombBlock => honeycomb_block
    BlockofNetherite => netherite_block
    AncientDebris => ancient_debris
    CryingObsidian => crying_obsidian
    RespawnAnchor => respawn_anchor
    PottedCrimsonFungus => potted_crimson_fungus
    PottedWarpedFungus => potted_warped_fungus
    PottedCrimsonRoots => potted_crimson_roots
    PottedWarpedRoots => potted_warped_roots
    Lodestone => lodestone
    Blackstone => blackstone
    BlackstoneStairs => blackstone_stairs
    BlackstoneWall => blackstone_wall
    BlackstoneSlab => blackstone_slab
    PolishedBlackstone => polished_blackstone
    PolishedBlackstoneBricks => polished_blackstone_bricks
    CrackedPolishedBlackstoneBricks => cracked_polished_blackstone_bricks
    ChiseledPolishedBlackstone => chiseled_polished_blackstone
    PolishedBlackstoneBrickSlab => polished_blackstone_brick_slab
    PolishedBlackstoneBrickStairs => polished_blackstone_brick_stairs
    PolishedBlackstoneBrickWall => polished_blackstone_brick_wall
    GildedBlackstone => gilded_blackstone
    PolishedBlackstoneStairs => polished_blackstone_stairs
    PolishedBlackstoneSlab => polished_blackstone_slab
    PolishedBlackstonePressurePlate => polished_blackstone_pressure_plate
    PolishedBlackstoneButton => polished_blackstone_button
    PolishedBlackstoneWall => polished_blackstone_wall
    ChiseledNetherBricks => chiseled_nether_bricks
    CrackedNetherBricks => cracked_nether_bricks
    QuartzBricks => quartz_bricks
    Candle => candle
    WhiteCandle => white_candle
    OrangeCandle => orange_candle
    MagentaCandle => magenta_candle
    LightBlueCandle => light_blue_candle
    YellowCandle => yellow_candle
    LimeCandle => lime_candle
    PinkCandle => pink_candle
    GrayCandle => gray_candle
    LightGrayCandle => light_gray_candle
    CyanCandle => cyan_candle
    PurpleCandle => purple_candle
    BlueCandle => blue_candle
    BrownCandle => brown_candle
    GreenCandle => green_candle
    RedCandle => red_candle
    BlackCandle => black_candle
    CakewithCandle => candle_cake
    CakewithWhiteCandle => white_candle_cake
    CakewithOrangeCandle => orange_candle_cake
    CakewithMagentaCandle => magenta_candle_cake
    CakewithLightBlueCandle => light_blue_candle_cake
    CakewithYellowCandle => yellow_candle_cake
    CakewithLimeCandle => lime_candle_cake
    CakewithPinkCandle => pink_candle_cake
    CakewithGrayCandle => gray_candle_cake
    CakewithLightGrayCandle => light_gray_candle_cake
    CakewithCyanCandle => cyan_candle_cake
    CakewithPurpleCandle => purple_candle_cake
    CakewithBlueCandle => blue_candle_cake
    CakewithBrownCandle => brown_candle_cake
    CakewithGreenCandle => green_candle_cake
    CakewithRedCandle => red_candle_cake
    CakewithBlackCandle => black_candle_cake
    BlockofAmethyst => amethyst_block
    BuddingAmethyst => budding_amethyst
    AmethystCluster => amethyst_cluster
    LargeAmethystBud => large_amethyst_bud
    MediumAmethystBud => medium_amethyst_bud
    SmallAmethystBud => small_amethyst_bud
    Tuff => tuff
    Calcite => calcite
    TintedGlass => tinted_glass
    PowderSnow => powder_snow
    SculkSensor => sculk_sensor
    CalibratedSculkSensor => calibrated_sculk_sensor
    Sculk => sculk
    SculkVein => sculk_vein
    SculkCatalyst => sculk_catalyst
    SculkShrieker => sculk_shrieker
    OxidizedCopper => oxidized_copper
    WeatheredCopper => weathered_copper
    ExposedCopper => exposed_copper
    BlockofCopper => copper_block
    CopperOre => copper_ore
    DeepslateCopperOre => deepslate_copper_ore
    OxidizedCutCopper => oxidized_cut_copper
    WeatheredCutCopper => weathered_cut_copper
    ExposedCutCopper => exposed_cut_copper
    CutCopper => cut_copper
    OxidizedCutCopperStairs => oxidized_cut_copper_stairs
    WeatheredCutCopperStairs => weathered_cut_copper_stairs
    ExposedCutCopperStairs => exposed_cut_copper_stairs
    CutCopperStairs => cut_copper_stairs
    OxidizedCutCopperSlab => oxidized_cut_copper_slab
    WeatheredCutCopperSlab => weathered_cut_copper_slab
    ExposedCutCopperSlab => exposed_cut_copper_slab
    CutCopperSlab => cut_copper_slab
    WaxedBlockofCopper => waxed_copper_block
    WaxedWeatheredCopper => waxed_weathered_copper
    WaxedExposedCopper => waxed_exposed_copper
    WaxedOxidizedCopper => waxed_oxidized_copper
    WaxedOxidizedCutCopper => waxed_oxidized_cut_copper
    WaxedWeatheredCutCopper => waxed_weathered_cut_copper
    WaxedExposedCutCopper => waxed_exposed_cut_copper
    WaxedCutCopper => waxed_cut_copper
    WaxedOxidizedCutCopperStairs => waxed_oxidized_cut_copper_stairs
    WaxedWeatheredCutCopperStairs => waxed_weathered_cut_copper_stairs
    WaxedExposedCutCopperStairs => waxed_exposed_cut_copper_stairs
    WaxedCutCopperStairs => waxed_cut_copper_stairs
    WaxedOxidizedCutCopperSlab => waxed_oxidized_cut_copper_slab
    WaxedWeatheredCutCopperSlab => waxed_weathered_cut_copper_slab
    WaxedExposedCutCopperSlab => waxed_exposed_cut_copper_slab
    WaxedCutCopperSlab => waxed_cut_copper_slab
    LightningRod => lightning_rod
    PointedDripstone => pointed_dripstone
    DripstoneBlock => dripstone_block
    CaveVines => cave_vines
    CaveVinesPlant => cave_vines_plant
    SporeBlossom => spore_blossom
    Azalea => azalea
    FloweringAzalea => flowering_azalea
    MossCarpet => moss_carpet
    PinkPetals => pink_petals
    MossBlock => moss_block
    BigDripleaf => big_dripleaf
    BigDripleafStem => big_dripleaf_stem
    SmallDripleaf => small_dripleaf
    HangingRoots => hanging_roots
    RootedDirt => rooted_dirt
    Mud => mud
    Deepslate => deepslate
    CobbledDeepslate => cobbled_deepslate
    CobbledDeepslateStairs => cobbled_deepslate_stairs
    CobbledDeepslateSlab => cobbled_deepslate_slab
    CobbledDeepslateWall => cobbled_deepslate_wall
    PolishedDeepslate => polished_deepslate
    PolishedDeepslateStairs => polished_deepslate_stairs
    PolishedDeepslateSlab => polished_deepslate_slab
    PolishedDeepslateWall => polished_deepslate_wall
    DeepslateTiles => deepslate_tiles
    DeepslateTileStairs => deepslate_tile_stairs
    DeepslateTileSlab => deepslate_tile_slab
    DeepslateTileWall => deepslate_tile_wall
    DeepslateBricks => deepslate_bricks
    DeepslateBrickStairs => deepslate_brick_stairs
    DeepslateBrickSlab => deepslate_brick_slab
    DeepslateBrickWall => deepslate_brick_wall
    ChiseledDeepslate => chiseled_deepslate
    CrackedDeepslateBricks => cracked_deepslate_bricks
    CrackedDeepslateTiles => cracked_deepslate_tiles
    InfestedDeepslate => infested_deepslate
    SmoothBasalt => smooth_basalt
    BlockofRawIron => raw_iron_block
    BlockofRawCopper => raw_copper_block
    BlockofRawGold => raw_gold_block
    PottedAzalea => potted_azalea_bush
    PottedFloweringAzalea => potted_flowering_azalea_bush
    OchreFroglight => ochre_froglight
    VerdantFroglight => verdant_froglight
    PearlescentFroglight => pearlescent_froglight
    Frogspawn => frogspawn
    ReinforcedDeepslate => reinforced_deepslate
    DecoratedPot => decorated_pot
);
