use phf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
  Float,
  Long,
  String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResType {
  Float,
  Long,
  Void,
}

mwfuncs! {
  Disable(| s): v,
  Enable(| s): v,
  GetDisabled(| s): l,
  GetDistance(s): f,
  GetSecondsPassed(): f,
  MenuMode(): l,
  MessageBox(s | s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s s): v,
  Random(l): l,
  Return(): v,
  ScriptRunning(s): l,
  StartScript(s): v,
  StopScript(s): v,
  // AI
  AIActivate(s | l): v,
  AIEscort(s f f f f | l): v,
  AIEscortCell(s s f f f f | l): v,
  AIFollow(s f f f f | l l l l l l l l): v,
  AIFollowCell(s s f f f f | l): v,
  AITravel(f f f | l): v,
  AIWander(f f f | l l l l l l l l l l): v,
  Face(f f | f): v, // TODO: extra arg?
  GetAIPackageDone(): l,
  GetAlarm(): l,
  GetCurrentAIPackage(): l,
  GetDetected(s): l,
  GetFight(): l,
  GetFlee(): l,
  GetHello(): l,
  GetLOS(s): l,
  GetLineOfSight(s): l,
  GetTarget(s): l,
  ModAlarm(l): v,
  ModFight(l): v,
  ModFlee(l): v,
  ModHello(l): v,
  SetAlarm(l): v,
  SetFight(l): v,
  SetFlee(l): v,
  SetHello(l): v,
  StartCombat(s): v,
  StopCombat(| s): v,
  TAI(): v,
  ToggleAI(): v,
  // Animation
  LoopGroup(s l | l): v,
  PlayGroup(s | l): v,
  SkipAnim(): v,
  // Cell
  COC(s): v,
  COE(l l): v,
  CellChanged(): l,
  CenterOnCell(s): v,
  CenterOnExterior(l l): v,
  GetInterior(): l,
  GetPCCell(s): l,
  GetWaterLevel(): f,
  ModWaterLevel(f): v,
  SetWaterLevel(f): v,
  // Container
  AddItem(s l): v,
  Equip(s): v,
  GetArmorType(l): l,
  GetItemCount(s): l,
  GetWeaponType(): l,
  HasItemEquipped(s): l,
  HasSoulgem(s): l,
  RemoveItem(s l): v,
  // Control
  ClearForceJump(): v,
  ClearForceMoveJump(): v,
  ClearForceRun(): v,
  ClearForceSneak(): v,
  DisablePlayerControls(): v,
  DisablePlayerFighting(): v,
  DisablePlayerJumping(): v,
  DisablePlayerLooking(): v,
  DisablePlayerMagic(): v,
  DisablePlayerViewSwitch(): v,
  DisableVanityMode(): v,
  EnablePlayerControls(): v,
  EnablePlayerFighting(): v,
  EnablePlayerJumping(): v,
  EnablePlayerLooking(): v,
  EnablePlayerMagic(): v,
  EnablePlayerViewSwitch(): v,
  EnableVanityMode(): v,
  ForceJump(): v,
  ForceMoveJump(): v,
  ForceRun(): v,
  ForceSneak(): v,
  GetForceJump(): l,
  GetForceMoveJump(): l,
  GetForceRun(): l,
  GetForceSneak(): l,
  GetPCRunning(): l,
  GetPCSneaking(): l,
  GetPlayerControls(): l,
  GetPlayerFighting(): l,
  GetPlayerJumping(): l,
  GetPlayerLooking(): l,
  GetPlayerMagic(): l,
  GetPlayerViewSwitch(): l,
  GetVanityMode(): l,
  TCL(): v,
  ToggleCollision(): v,
  // Dialogue
  AddTopic(s): v,
  Choice(| s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l s l): v,
  ClearInfoActor(): v,
  ForceGreeting(): v,
  GetFactionReaction(s s | l): l,
  GetJournalIndex(s): l,
  GetReputation(): l,
  Goodbye(): v,
  Journal(s l): v,
  ModFactionReaction(s s l): v,
  ModReputation(l): v,
  SameFaction(): l,
  SetFactionReaction(s s l): v,
  SetJournalIndex(s l): v,
  SetReputation(l): v,
  // Gui
  EnableBirthMenu(): v,
  EnableClassMenu(): v,
  EnableInventoryMenu(): v,
  EnableLevelupMenu(): v,
  EnableMagicMenu(): v,
  EnableMapMenu(): v,
  EnableNameMenu(): v,
  EnableRaceMenu(): v,
  EnableRest(): v,
  EnableStatReviewMenu(): v,
  EnableStatsMenu(): v,
  FillMap(): v,
  GetButtonPressed(): l,
  MenuTest(| l): v,
  ShowMap(s): v,
  ShowRestMenu(): v,
  TFH(): v,
  TFOW(): v,
  TM(): v,
  ToggleFogOfWar(): v,
  ToggleFullHelp(): v,
  ToggleMenus(): v,
  // Misc
  Activate(): v,
  AddSoulGem(s s): v,
  AddToLevCreature(s s l): v,
  AddToLevItem(s s l): v,
  BC(| s): v,
  BetaComment(| s): v,
  Cast(s s): v,
  DisableLevitation(): v,
  DisableTeleporting(): v,
  DontSaveObject(): v,
  Drop(s l): v,
  DropSoulGem(s): v,
  EnableLevitation(): v,
  EnableTeleporting(): v,
  ExplodeSpell(s): v,
  FadeIn(f): v,
  FadeOut(f): v,
  FadeTo(f f): v,
  Fall(): v,
  GetAttacked(): l,
  GetCollidingActor(): l,
  GetCollidingPC(): l,
  GetCurrentTime(): f,
  GetEffect(s): l,
  GetLocked(): l,
  GetPCInJail(): l,
  GetPCJumping(): l,
  GetPCSleep(): l,
  GetPCTraveling(): l,
  GetSpellEffects(s): l,
  GetSpellReadied(): l,
  GetSquareRoot(f): f,
  GetStandingActor(): l,
  GetStandingPC(): l,
  GetWeaponDrawn(): l,
  GetWindSpeed(): f,
  GoToJail(): v,
  HitAttemptOnMe(s): l,
  HitOnMe(s): l,
  HurtCollidingActor(f): v,
  HurtStandingActor(f): v,
  Lock(| l): v,
  ORI(| s): v,
  OnActivate(): l,
  PCForce1stPerson(): v,
  PCForce3rdPerson(): v,
  PCGet3rdPerson(): l,
  PayFine(): v,
  PayFineThief(): v,
  PlayBink(s l): v,
  RemoveFromLevCreature(s s l): v,
  RemoveFromLevItem(s s l): v,
  RemoveSoulGem(s | l): v,
  SSG(| l): v,
  SV(): v,
  SetDelete(l): v,
  Show(s): v,
  ShowSceneGraph(| l): v,
  ShowVars(): v,
  TB(): v,
  TCB(): v,
  TCG(): v,
  TGM(): v,
  TPG(): v,
  TVM(): v,
  TW(): v,
  TWA(): v,
  TWF(): v,
  ToggleBorders(): v,
  ToggleCollisionBoxes(): v,
  ToggleCollisionGrid(): v,
  ToggleGodMode(): v,
  TogglePathGrid(): v,
  ToggleScripts(): v,
  ToggleVanityMode(): v,
  ToggleWater(): v,
  ToggleWireFrame(): v,
  ToggleWorld(): v,
  Unlock(): v,
  WakeUpPC(): v,
  XBox(): l,
  // Sky
  ChangeWeather(s l): v,
  GetCurrentWeather(): l,
  GetMasserPhase(): l,
  GetSecundaPhase(): l,
  ModRegion(s | l l l l l l l l l l): v,
  TS(): v,
  ToggleSky(): v,
  TurnMoonRed(): v,
  TurnMoonWhite(): v,
  // Sound
  GetSoundPlaying(s): l,
  PlayLoopSound3D(s): v,
  PlayLoopSound3DVP(s f f): v,
  PlaySound(s): v,
  PlaySound3D(s): v,
  PlaySound3DVP(s f f): v,
  PlaySoundVP(s f f): v,
  Say(s s): v,
  SayDone(): l,
  StopSound(s): v,
  StreamMusic(s): v,
  // Stats
  AddSpell(s): v,
  BecomeWerewolf(): v,
  GetAcrobatics(): l,
  GetAgility(): l,
  GetAlchemy(): l,
  GetAlteration(): l,
  GetArmorBonus(): l,
  GetArmorer(): l,
  GetAthletics(): l,
  GetAttackBonus(): l,
  GetAxe(): l,
  GetBlightDisease(): l,
  GetBlindness(): l,
  GetBlock(): l,
  GetBluntWeapon(): l,
  GetCastPenalty(): l,
  GetChameleon(): l,
  GetCommonDisease(): l,
  GetConjuration(): l,
  GetDeadCount(s): l,
  GetDefendBonus(): l,
  GetDestruction(): l,
  GetDisposition(): l,
  GetEnchant(): l,
  GetEndurance(): l,
  GetFatigue(): f,
  GetFatigueGetRatio(): f,
  GetFlying(): l,
  GetHandToHand(): l,
  GetHealth(): f,
  GetHealthGetRatio(): f,
  GetHeavyArmor(): l,
  GetIllusion(): l,
  GetIntelligence(): l,
  GetInvisible(): l,
  GetLevel(): l,
  GetLightarmor(): l,
  GetLongBlade(): l,
  GetLuck(): l,
  GetMagicka(): f,
  GetMagickaGetRatio(): f,
  GetMarksman(): l,
  GetMediumArmor(): l,
  GetMercantile(): l,
  GetMysticism(): l,
  GetPCCrimeLevel(): f,
  GetPCFacRep(| s): l,
  GetPCRank(| s): l,
  GetParalysis(): l,
  GetPersonality(): l,
  GetRace(s): l,
  GetResistBlight(): l,
  GetResistCorprus(): l,
  GetResistDisease(): l,
  GetResistFire(): l,
  GetResistFrost(): l,
  GetResistMagicka(): l,
  GetResistNormalWeapons(): l,
  GetResistParalysis(): l,
  GetResistPoison(): l,
  GetResistShock(): l,
  GetRestoration(): l,
  GetSecurity(): l,
  GetShortBlade(): l,
  GetSilence(): l,
  GetSneak(): l,
  GetSpear(): l,
  GetSpeechcraft(): l,
  GetSpeed(): l,
  GetSpell(s): l,
  GetStat(s): l,
  GetStrength(): l,
  GetSuperJump(): l,
  GetSwimSpeed(): l,
  GetUnarmored(): l,
  GetWaterBreathing(): l,
  GetWaterWalking(): l,
  GetWerewolfKills(): l,
  GetWillpower(): l,
  IsWerewolf(): l,
  LowerRank(): v,
  ModAcrobatics(l): v,
  ModAgility(l): v,
  ModAlchemy(l): v,
  ModAlteration(l): v,
  ModArmorBonus(l): v,
  ModArmorer(l): v,
  ModAthletics(l): v,
  ModAttackBonus(l): v,
  ModAxe(l): v,
  ModBlindness(l): v,
  ModBlock(l): v,
  ModBluntWeapon(l): v,
  ModCastPenalty(l): v,
  ModChameleon(l): v,
  ModConjuration(l): v,
  ModCurrentFatigue(f): v,
  ModCurrentHealth(f): v,
  ModCurrentMagicka(f): v,
  ModDefendBonus(l): v,
  ModDestruction(l): v,
  ModDisposition(l): v,
  ModEnchant(l): v,
  ModEndurance(l): v,
  ModFatigue(f): v,
  ModFlying(l): v,
  ModHandToHand(l): v,
  ModHealth(f): v,
  ModHeavyArmor(l): v,
  ModIllusion(l): v,
  ModIntelligence(l): v,
  ModInvisible(l): v,
  ModLightarmor(l): v,
  ModLongBlade(l): v,
  ModLuck(l): v,
  ModMagicka(f): v,
  ModMarksman(l): v,
  ModMediumArmor(l): v,
  ModMercantile(l): v,
  ModMysticism(l): v,
  ModPCCrimeLevel(f): v,
  ModPCFacRep(l | s): v,
  ModParalysis(l): v,
  ModPersonality(l): v,
  ModResistBlight(l): v,
  ModResistCorprus(l): v,
  ModResistDisease(l): v,
  ModResistFire(l): v,
  ModResistFrost(l): v,
  ModResistMagicka(l): v,
  ModResistNormalWeapons(l): v,
  ModResistParalysis(l): v,
  ModResistPoison(l): v,
  ModResistShock(l): v,
  ModRestoration(l): v,
  ModSecurity(l): v,
  ModShortBlade(l): v,
  ModSilence(l): v,
  ModSneak(l): v,
  ModSpear(l): v,
  ModSpeechcraft(l): v,
  ModSpeed(l): v,
  ModStrength(l): v,
  ModSuperJump(l): v,
  ModSwimSpeed(l): v,
  ModUnarmored(l): v,
  ModWaterBreathing(l): v,
  ModWaterWalking(l): v,
  ModWillpower(l): v,
  OnDeath(): l,
  OnKnockout(): l,
  OnMurder(): l,
  PCClearExpelled(| s): v,
  PCExpell(| s): v,
  PCExpelled(| s): l,
  PCJoinFaction(| s): v,
  PCLowerRank(| s): v,
  PCRaiseRank(| s): v,
  RaiseRank(): v,
  RemoveEffects(l): v,
  RemoveSpell(s): v,
  RemoveSpellEffects(s): v,
  Resurrect(): v,
  SetAcrobatics(l): v,
  SetAgility(l): v,
  SetAlchemy(l): v,
  SetAlteration(l): v,
  SetArmorBonus(l): v,
  SetArmorer(l): v,
  SetAthletics(l): v,
  SetAttackBonus(l): v,
  SetAxe(l): v,
  SetBlindness(l): v,
  SetBlock(l): v,
  SetBluntWeapon(l): v,
  SetCastPenalty(l): v,
  SetChameleon(l): v,
  SetConjuration(l): v,
  SetDefendBonus(l): v,
  SetDestruction(l): v,
  SetDisposition(l): v,
  SetEnchant(l): v,
  SetEndurance(l): v,
  SetFatigue(f): v,
  SetFlying(l): v,
  SetHandToHand(l): v,
  SetHealth(f): v,
  SetHeavyArmor(l): v,
  SetIllusion(l): v,
  SetIntelligence(l): v,
  SetInvisible(l): v,
  SetLevel(l): v,
  SetLightarmor(l): v,
  SetLongBlade(l): v,
  SetLuck(l): v,
  SetMagicka(f): v,
  SetMarksman(l): v,
  SetMediumArmor(l): v,
  SetMercantile(l): v,
  SetMysticism(l): v,
  SetPCCrimeLevel(f): v,
  SetPCFacRep(l | s): v,
  SetParalysis(l): v,
  SetPersonality(l): v,
  SetResistBlight(l): v,
  SetResistCorprus(l): v,
  SetResistDisease(l): v,
  SetResistFire(l): v,
  SetResistFrost(l): v,
  SetResistMagicka(l): v,
  SetResistNormalWeapons(l): v,
  SetResistParalysis(l): v,
  SetResistPoison(l): v,
  SetResistShock(l): v,
  SetRestoration(l): v,
  SetSecurity(l): v,
  SetShortBlade(l): v,
  SetSilence(l): v,
  SetSneak(l): v,
  SetSpear(l): v,
  SetSpeechcraft(l): v,
  SetSpeed(l): v,
  SetStrength(l): v,
  SetSuperJump(l): v,
  SetSwimSpeed(l): v,
  SetUnarmored(l): v,
  SetWaterBreathing(l): v,
  SetWaterWalking(l): v,
  SetWerewolfAcrobatics(): v,
  SetWillpower(l): v,
  UndoWerewolf(): v,
  // Transformation
  FixMe(): v,
  GetAngle(s): f,
  GetPos(s): f,
  GetScale(): f,
  GetStartingAngle(s): f,
  GetStartingPos(s): f,
  ModScale(f): v,
  Move(s f): v,
  MoveWorld(s f): v,
  PlaceAtMe(s l f l): v,
  PlaceAtPC(s l f l): v,
  PlaceItem(s f f f f): v,
  PlaceItemCell(s s f f f f): v,
  Position(f f f f): v,
  PositionCell(f f f f s): v,
  RA(): v,
  ResetActors(): v,
  Rotate(s f): v,
  RotateWorld(s f): v,
  SetAngle(s f): v,
  SetAtStart(): v,
  SetPos(s f): v,
  SetScale(f): v
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
