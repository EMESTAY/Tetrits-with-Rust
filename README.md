# Rust Tetris 🍮

A Tetris clone built with **Rust** and **Macroquad**.

## 🛠️ Technologies

- **[Rust](https://www.rust-lang.org/)**: Core programming language.
- **[Macroquad](https://macroquad.rs/)**: Simple and fast game library for Rust.
- **Rand**: For random piece generation.

## 🎮 How to Run

Prerequisites: Ensure you have [Rust installed](https://rustup.rs/).

```bash
# Run the game
cargo run
```

## 🕹️ Controls

- **⬅️ Left / ➡️ Right**: Move Piece
- **⬆️ Up**: Rotate Piece
- **⬇️ Down**: Soft Drop
- **Space**: Hard Drop
- **C**: Hold Piece
- **R**: Reset Game (on Game Over)

## 🎨 Colors

- **I**: Cyan (`#00E0FF`)
- **O**: Golden Yellow (`#FFD400`)
- **T**: Rich Purple (`#9D51F3`)
- **S**: Lime Green (`#71F55F`)
- **Z**: Coral Red (`#FF4E50`)
- **J**: Strong Blue (`#2E86AB`)
- **L**: Bright Orange (`#FF9F1C`)
- **Plus (+)**: Hot Pink (`#FF69B4`)
- **Jelly (Special)**: Pink (`#FB6F92`)
- **Bomb (Special)**: Lava Orange (`#F35B04`)

## 🎁 Bonuses

Unlock bonuses by clearing lines and leveling up!

### Common

- 💣 **BOMB BLOCK**: Explodes on impact, clearing a 3x3 area.
- ❄️ **CHILL TIME**: Slows time by 50% for 60 seconds.
- ⚡ **LASER BEAM**: Clears the entire column on lock.
- 🍮 **JELLY BIDULE**: Drops 6 liquid blocks that fill the lowest gaps.
- 🔨 **DRILL**: Next piece smashes through blocks to the bottom.

### Rare

- ⚓ **TIME ANCHOR**: Passively slows gravity by 10%. Stacks.
- ⛏️ **GOLD PICKAXE**: +20% Score gained from lines. Stacks.

### Legendary

- 💖 **LIFE INSURANCE**: Prevents Game Over once. Consumable.

## 👨‍💻 Developer Mode

Access the developer menu to test all bonuses:

- Click the **Sun** ☀️ in the top-left corner of the background.

---

# Rust Tetris

Un clone de Tetris développé avec **Rust** et **Macroquad**.

## 🛠️ Technologies Utilisées

- **[Rust](https://www.rust-lang.org/)**: Langage de programmation principal.
- **[Macroquad](https://macroquad.rs/)**: Bibliothèque de jeux simple et rapide pour Rust.

## 🎮 Comment Jouer

Prérequis : Assurez-vous d'avoir [installé Rust](https://rustup.rs/).

```bash
# Lancer le jeu
cargo run
```

## 🕹️ Contrôles

- **⬅️ Gauche / ➡️ Droite** : Déplacer la pièce
- **⬆️ Haut** : Pivoter la pièce
- **⬇️ Bas** : Chute douce
- **Espace** : Chute rapide
- **C** : Garder la pièce
- **R** : Recommencer la partie (Écran Game Over)

## 🎨 Couleurs

- **I** : Cyan (`#00E0FF`)
- **O** : Jaune Doré (`#FFD400`)
- **T** : Violet Riche (`#9D51F3`)
- **S** : Vert Citron (`#71F55F`)
- **Z** : Rouge Corail (`#FF4E50`)
- **J** : Bleu Fort (`#2E86AB`)
- **L** : Orange Vif (`#FF9F1C`)
- **Plus (+)** : Rose Vif (`#FF69B4`)
- **Gelée (Spécial)** : Rose (`#FB6F92`)
- **Bombe (Spécial)** : Orange Lave (`#F35B04`)

## 🎁 Bonus

Débloquez des bonus en effaçant des lignes et en montant de niveau !

### Communs

- 💣 **BLOC BOMBE** : Explose à l'impact, nettoyant une zone de 3x3.
- ❄️ **TEMPS GELÉ** : Ralentit le temps de 50% pendant 60 secondes.
- ⚡ **RAYON LASER** : Efface toute la colonne lors du verrouillage.
- 🍮 **BIDULE GELÉE** : Lâche 6 blocs liquides qui comblent les trous les plus bas.
- 🔨 **FOREUSE** : La prochaine pièce traverse les blocs jusqu'en bas.

### Rares

- ⚓ **ANCRE TEMPORELLE** : Ralentit passivement la gravité de 10%. Cumulable.
- ⛏️ **PIOCHE EN OR** : +20% de score gagné par ligne. Cumulable.

### Légendaires

- 💖 **ASSURANCE VIE** : Empêche le Game Over une fois. Consommable.

## 👨‍💻 Mode Développeur

Accédez au menu développeur pour tester tous les bonus :

- Cliquez sur le **Soleil** ☀️ dans le coin supérieur gauche de l'arrière-plan.
