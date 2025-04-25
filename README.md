# 8-Puzzle Solver Partie complèmentaire  (Rust)

Ce projet implémente plusieurs algorithmes pour résoudre le problème du 8‑puzzle (et au-delà) en Rust :

- **A\*** (min‑heap) pour des plateaux de taille raisonnable.
- **IDA\*** (Iterative Deepening A\*) pour une recherche en profondeur limitée en mémoire.
- **Beam Search** pour une recherche heuristique à faisceau restreint.

## Fonctionnalités complémentaires

1. **Génération d’instances aléatoires**  
   - Méthode `Board::random(shuffle_moves: usize)` qui effectue `shuffle_moves` coups aléatoires à partir de l’état objectif, garantissant la solvabilité.

2. **Support dynamique de la taille `N × N`**
   - Constante `N` modifiable dans `board.rs` (par défaut 3 pour le 8‑puzzle, 4 pour le 15‑puzzle).
   - Affichage et algorithmes qui s’adaptent automatiquement à tout `N`.

3. **Algorithmes « mémoire limitée »**
   - **IDA\*** dans `search.rs` : explore par profondeur itérative, sans stocker tout l’espace de recherche.
   - **Beam Search** dans `search.rs` : conserve à chaque profondeur un nombre fixe d’états (`beam_width`).






