-- Formalização do Teorema das Quatro Cores (4CT) - Fronteira Sub-30
-- Definições fundamentais de configurações planares com bordo e contratos de Birkhoff

namespace FourColor

/-- Configuração planar triangulada com anel de fronteira e vértices interiores. -/
structure Configuration where
  id : Nat
  name : String
  verts : Nat
  ring : Nat
  contracts : List (Nat × Nat)
  adj : List (Nat × List Nat)
  deriving Repr, DecidableEq

/-- Verifica se o contrato de contração satisfaz k <= 2. -/
def Configuration.contractDepthLe2 (c : Configuration) : Bool :=
  c.contracts.length <= 2

/-- Verifica se a configuração é D-redutível (extensão direta de Kempe, k = 0). -/
def Configuration.isD (c : Configuration) : Bool :=
  c.contracts.isEmpty

/-- Verifica se a configuração é C-redutível (k > 0 arestas contraídas). -/
def Configuration.isC (c : Configuration) : Bool :=
  !c.contracts.isEmpty

/-- Verifica se todos os vértices interiores (v > ring) possuem grau >= 5. -/
def Configuration.minInteriorDegree5 (c : Configuration) : Bool :=
  c.adj.all fun (v, nbs) =>
    if v > c.ring then nbs.length >= 5 else true

/-- Verifica se o anel de fronteira é chordless (sem arestas entre nós não adjacentes do anel). -/
def Configuration.isChordlessRing (c : Configuration) : Bool :=
  c.adj.all fun (v, nbs) =>
    if v <= c.ring then
      nbs.all fun w =>
        if w <= c.ring then
          w == v + 1 || (v == c.ring && w == 1) || (w + 1 == v) || (w == c.ring && v == 1)
        else
          true
    else
      true

/-- Verifica consistência estrutural: total de vértices e anel válido. -/
def Configuration.isWellFormed (c : Configuration) : Bool :=
  c.verts >= c.ring && c.ring >= 5 && c.adj.length == c.verts &&
  c.contractDepthLe2 && c.minInteriorDegree5 && c.isChordlessRing

end FourColor
