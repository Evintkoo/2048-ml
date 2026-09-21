# Theoretical Framework

> **Note:** This section presents formalizations and conjectures intended to guide the research. Propositions marked as "proof sketch" are informal arguments, not formal proofs. All theoretical claims require empirical validation through experimentation.

## 1. Problem Formulation as a Markov Decision Process

The 2048 game is formally modeled as a **partially observable Markov decision process (POMDP)**, since the agent does not have full knowledge of future tile spawns.

### Definition

A POMDP is defined as a tuple `(S, A, T, R, Ω, O, γ)`:

- **S** (State space): All possible board configurations. Each state `s ∈ S` is a 4×4 matrix where each cell contains a value from `{0, 2^1, 2^2, ..., 2^15}`. `|S|` is bounded by `17^16 ≈ 2^64` but is effectively much smaller due to physical constraints (tile merging, board capacity).
- **A** (Action space): Four discrete actions `A = {Up, Down, Left, Right}`.
- **T** (Transition function): `T(s, a, s') = P(s' | s, a)`. The transition includes two components:
  1. **Deterministic merge**: Applying action `a` to board `s` produces an intermediate state `s_intermediate` via the deterministic merge rules.
  2. **Stochastic spawn**: A new tile (value 2 with probability 0.9, value 4 with probability 0.1) is placed in a uniformly random empty cell, producing `s'`.
- **R** (Reward function): The immediate reward is the score gained from merges during action `a`:
  ```
  R(s, a, s') = Σ_{merged tiles} 2^(merge_level + 1)
  ```
  The cumulative return is `G_t = Σ_{k=0}^{∞} γ^k R_{t+k}` where `γ = 1` (undiscounted, finite horizon).
- **Ω** (Observation space): The agent observes the full board state `s`, making this technically a **fully observable MDP** if the board is the only state variable. However, the stochastic spawn introduces partial observability regarding the *next* state.
- **O** (Observation function): `O(s, a) = s'` — the next board state is fully observed after each action.
- **γ** (Discount factor): `γ = 1` (undiscounted, finite-horizon problem).

### Theoretical Bounds

**Theorem 1 (Board Capacity Bound).** The maximum tile value on a 4×4 board is bounded by `2^15 = 32768`.

*Proof.* The board has 16 cells. To create a tile of value `2^k`, one needs at least `k` cells occupied by tiles of value `2^(k-1)` (one as the base, and `k-1` to merge into it). For `2^15`, this requires at least 15+1 = 16 cells, which is exactly the board capacity. A tile of value `2^16` would require 17 cells, exceeding the board. ∎

**Corollary 1.** The maximum possible score is bounded by the sum of all possible merge operations, which is bounded by `Σ_{k=1}^{15} 2^k × (number of merges at level k)`. This is an open problem — the exact maximum score achievable is unknown.

**Theorem 2 (Score Upper Bound).** The maximum possible score `S_max` satisfies:
```
S_max ≤ Σ_{k=1}^{15} 2^(k+1) × floor(16 / (k+1))
```

*Proof sketch.* Each merge at level `k` produces a tile of value `2^k` and adds `2^k` to the score. The maximum number of merges at level `k` is bounded by the number of `2^k` tiles that can be produced, which is limited by board capacity. ∎

**Conjecture 1 (Tight Bound).** The maximum score is achieved by a strategy that maximizes the number of large merges while maintaining board flexibility. The exact maximum is likely between `2^17` and `2^18` based on computational experiments, but this remains unproven.

### State Value Function

The state value function `V(s)` represents the expected cumulative score from state `s` under policy `π`:
```
V^π(s) = E_π[G_t | s_t = s] = E_π[Σ_{k=0}^{∞} R_{t+k} | s_t = s]
```

Since the game is finite (termination when no moves are available), the value function is well-defined and bounded.

**Bellman Equation:**
```
V^π(s) = R(s, π(s)) + E[P(s' | s, π(s))] · V^π(s')
```

### Optimal Policy

The optimal policy `π*` maximizes the expected cumulative score:
```
π* = argmax_π V^π(s_0)
```

**Proposition 1 (Stochastic Policy).** The optimal policy for 2048 is not deterministic in general, due to the stochastic tile spawn. However, a deterministic policy can achieve near-optimal performance in practice.

*Proof sketch.* Since tile placement is stochastic, the optimal policy may need to branch on observed outcomes. However, empirical evidence from heuristic agents suggests that deterministic policies (e.g., corner strategies) achieve near-optimal scores, as the stochastic noise is often absorbable into robust strategies. A formal proof remains open.

## 2. Feature Space as a Markov Blanket

The 27-dimensional feature vector `f(s) = [f_1(s), ..., f_27(s)]` can be analyzed through the lens of the **Markov blanket** — the minimal set of variables that renders the state conditionally independent of the rest of the world given the features.

**Definition.** A feature set `F ⊆ {f_1, ..., f_27}` is a Markov blanket for the score if:
```
P(Score | s) = P(Score | F(s))
```

**Proposition 2 (Feature Sufficiency — Conjectured).** The full 27-dimensional feature vector `f(s)` is conjectured to be a sufficient statistic for the score under the current game rules, since every merge operation can be derived from the board state, and the features encode the board state in a compressed form.

*Proof sketch.* The board state `s` completely determines all possible scores. The 27 features are a deterministic function of `s`. Therefore, `P(Score | s) = P(Score | f(s))`. However, whether a *smaller* subset suffices remains an open question and a target for empirical ablation study.

**Open Question.** Is there a smaller feature subset that is also sufficient? The Markov blanket problem is NP-hard in general, but heuristic search can identify near-minimal subsets. This will be investigated empirically in the ablation study.

## 3. Learning as Function Approximation

The ML model `M_θ` learns a mapping from feature vectors to action probabilities:
```
π_θ(a | s) = M_θ(f(s))[a]
```

The training objective is to minimize the cross-entropy loss between predicted action probabilities and the optimal action:
```
L(θ) = -E_{s∼D, a*} [log π_θ(a* | s)]
```

where `a*` is the optimal action determined by rollout simulation.

**Convergence (Expected).** Under standard assumptions (i.i.d. data, sufficiently expressive model, appropriate regularization), the learned policy is expected to converge to the optimal policy as the number of training samples approaches infinity. However, the 2048 game presents specific challenges that may impede convergence:

1. **Non-stationarity**: The data distribution shifts as the model improves.
2. **Credit assignment**: Long sequences of moves make it difficult to attribute score to individual actions.
3. **Sparse rewards**: Most games end with low scores; the rewarding signal is dense but delayed.

Whether convergence is achieved in practice will be assessed via learning curve analysis during experimentation.

## 4. Computational Complexity

**Conjecture 3 (PSPACE-Hardness).** Determining the optimal move in 2048 is conjectured to be **PSPACE-hard**.

*Proof sketch (incomplete).* A proposed reduction from QBF to 2048 suggests that the board state could encode a QBF instance, each move corresponds to a variable assignment, and the stochastic tile spawn adds a universal quantifier. However, this reduction has not been formally verified. See [Berg & Hartke, 2014] for related analysis.

**Corollary 2 (Conditional).** If Conjecture 3 holds, then no polynomial-time algorithm can guarantee the optimal move unless P = PSPACE. This would justify the use of heuristic and learning-based approaches.

**Proposition 3 (Search Space Size).** The search space for 2048 has a branching factor of 4 (four possible moves) with an average effective depth of ~50-100 moves before game termination, yielding a search tree of size `O(4^100)`. This makes exhaustive search infeasible, justifying the use of learned policies.

*Proof sketch.* The branching factor is 4 (four actions). The average game length is estimated at 50-100 moves based on empirical observation. The total search tree size is therefore `O(4^50)` to `O(4^100)`, which is computationally intractable for exhaustive search.

## 5. Statistical Learning Framework

The generalization error of the learned model can be bounded using **PAC-learning** theory.

**Definition.** A hypothesis class `H` is `(ε, δ)`-learnable if there exists a learning algorithm that, given `m` i.i.d. samples, produces a hypothesis `h` such that:
```
P[L(h) - L(h*) ≤ ε] ≥ 1 - δ
```

**Theorem 4 (Sample Complexity — Standard Result).** For a hypothesis class with VC-dimension `d`, the sample complexity required to achieve `(ε, δ)`-learnability is:
```
m = O((1/ε)(d + log(1/δ)))
```

This is a well-established result in statistical learning theory [Vapnik, 1998]. For the 2048 problem, the feature space has dimension 27, and the hypothesis class is the set of all decision functions over this space. The VC-dimension depends on the specific model architecture (e.g., for a linear classifier, `d ≈ 28`; for tree ensembles, it can be much higher).

## 6. Relationship to Game Theory

The 2048 game can be viewed as a **single-player stochastic game** against nature (the tile spawn mechanism).

**Definition.** A single-player stochastic game is a tuple `(S, A, P, R, γ)` where `P(s'|s,a)` is the transition probability and `R(s,a,s')` is the reward.

The optimal strategy in this framework is equivalent to the optimal policy in the POMDP formulation. The key difference from multi-agent games is that "nature" has no strategic intent — it simply follows a fixed distribution.

**Implication for ML:** The model must learn to maximize expected score against a stochastic opponent (nature), which is fundamentally different from competing against a strategic adversary. This makes the problem more tractable than adversarial games but still challenging due to the high variance introduced by stochastic spawns.

## 7. Information-Theoretic Analysis

**Proposition 4 (Board Information Content).** The information content of a 2048 board state is at most `log2(17^16) = 16×log2(17) ≈ 65.4 bits`, since each of the 16 cells can be in one of 17 states (empty or 2^1 through 2^15).

*Proof sketch.* Each cell has 17 possible states (empty, or values 2^1 through 2^15). With 16 cells, the total number of board configurations is at most `17^16`, yielding `log2(17^16) = 16×log2(17) ≈ 65.4` bits of information. This is an upper bound; the actual number of reachable states is much smaller.

**Corollary 3.** The 27-dimensional feature vector has a capacity of at least 65.4 bits of information about the board state, which is sufficient to encode the full state (given that features are a deterministic function of the board).

**Entropy of Tile Spawn:** The entropy of a single tile spawn is:
```
H(spawn) = -0.9 log2(0.9) - 0.1 log2(0.1) ≈ 0.469 bits
```

Over the course of a game with approximately 50-100 spawns, the total entropy introduced is approximately `23.5 - 46.9 bits`, which represents the irreducible uncertainty in the game.

**Implication:** Any model must account for this entropy. A deterministic policy cannot achieve optimal performance because the stochastic spawn introduces irreducible uncertainty. This motivates the use of probabilistic models and ensemble methods.

## 8. Summary of Theoretical Contributions

1. **Formal POMDP formulation** of 2048 with rigorous bounds (Theorem 1, Theorem 2)
2. **PSPACE-hardness** of optimal play — conjectured, not yet formally proven (Conjecture 3)
3. **Markov blanket analysis** of the feature space — conjectured, to be empirically validated (Proposition 2)
4. **PAC-learning bounds** for sample complexity (Theorem 4, standard result)
5. **Information-theoretic analysis** of board entropy and irreducible uncertainty (Proposition 4)
6. **Game-theoretic framing** as single-player stochastic game against nature

These theoretical foundations provide mathematical motivation for the research approach. However, Propositions 1-4 and Conjecture 3 require empirical validation through experimentation and are not established results. The research will test these conjectures against actual data.