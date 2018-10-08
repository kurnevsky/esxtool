use esx::Esx;
use esx::record::Record;
use esx::record::tes3::Tes3Record;
use esx::record::tes3::sub_record::Tes3SubRecord;
use esx::record::gmst::GmstRecord;
use esx::record::gmst::sub_record::GmstSubRecord;
use esx::record::glob::GlobRecord;
use esx::record::glob::sub_record::GlobSubRecord;
use esx::record::clas::ClasRecord;
use esx::record::clas::sub_record::ClasSubRecord;
use esx::record::fact::FactRecord;
use esx::record::fact::sub_record::FactSubRecord;
use esx::record::race::RaceRecord;
use esx::record::race::sub_record::RaceSubRecord;
use esx::record::soun::SounRecord;
use esx::record::soun::sub_record::SounSubRecord;
use esx::record::skil::SkilRecord;
use esx::record::skil::sub_record::SkilSubRecord;
use esx::record::mgef::MgefRecord;
use esx::record::mgef::sub_record::MgefSubRecord;
use esx::record::scpt::ScptRecord;
use esx::record::scpt::sub_record::ScptSubRecord;
use esx::record::regn::RegnRecord;
use esx::record::regn::sub_record::RegnSubRecord;
use esx::record::bsgn::BsgnRecord;
use esx::record::bsgn::sub_record::BsgnSubRecord;
use esx::record::ltex::LtexRecord;
use esx::record::ltex::sub_record::LtexSubRecord;
use esx::record::stat::StatRecord;
use esx::record::stat::sub_record::StatSubRecord;
use esx::util::name_to_string;

macro_rules! print_field {
  ( $sub_record:ident . $field:ident , $format:expr) => {
    println!(concat!("\t\t", stringify!($field), ": ", $format), $sub_record.$field);
  };
  ( $sub_record:ident . $field:ident) => {
    print_field!($sub_record.$field, "{}")
  }
}

fn trace_tes3(tes3: &Tes3Record) {
  for (sub_record_number, sub_record) in tes3.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      Tes3SubRecord::Hedr(hedr) => {
        print_field!(hedr.file_type, "{:?}");
        print_field!(hedr.company_name);
        print_field!(hedr.file_description);
        print_field!(hedr.version);
        print_field!(hedr.num_records);
      },
      Tes3SubRecord::Mast(mast) => {
        print_field!(mast.master_file_name);
      },
      Tes3SubRecord::Data(data) => {
        print_field!(data.master_size);
      },
    }
  }
}

fn trace_gmst(gmst: &GmstRecord) {
  for (sub_record_number, sub_record) in gmst.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      GmstSubRecord::Name(name) => {
        print_field!(name.name);
      },
      GmstSubRecord::Strv(strv) => {
        print_field!(strv.value);
      },
      GmstSubRecord::Intv(intv) => {
        print_field!(intv.value);
      },
      GmstSubRecord::Fltv(fltv) => {
        print_field!(fltv.value);
      },
    }
  }
}

fn trace_glob(glob: &GlobRecord) {
  for (sub_record_number, sub_record) in glob.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      GlobSubRecord::Name(name) => {
        print_field!(name.name);
      },
      GlobSubRecord::Fnam(fnam) => {
        print_field!(fnam.global_type, "{:?}");
      },
      GlobSubRecord::Fltv(fltv) => {
        print_field!(fltv.value);
      },
    }
  }
}

fn trace_clas(clas: &ClasRecord) {
  for (sub_record_number, sub_record) in clas.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      ClasSubRecord::Name(name) => {
        print_field!(name.name);
      },
      ClasSubRecord::Fnam(fnam) => {
        print_field!(fnam.name);
      },
      ClasSubRecord::Cldt(cldt) => {
        print_field!(cldt.attribute_id_1);
        print_field!(cldt.attribute_id_2);
        print_field!(cldt.specialization, "{:?}");
        print_field!(cldt.minor_id_1);
        print_field!(cldt.major_id_1);
        print_field!(cldt.minor_id_2);
        print_field!(cldt.major_id_2);
        print_field!(cldt.minor_id_3);
        print_field!(cldt.major_id_3);
        print_field!(cldt.minor_id_4);
        print_field!(cldt.major_id_4);
        print_field!(cldt.minor_id_5);
        print_field!(cldt.major_id_5);
        print_field!(cldt.flags, "{:?}");
        print_field!(cldt.auto_calc_flags, "{:?}");
      },
      ClasSubRecord::Desc(desc) => {
        print_field!(desc.description);
      },
    }
  }
}

fn trace_fact(gmst: &FactRecord) {
  for (sub_record_number, sub_record) in gmst.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      FactSubRecord::Name(name) => {
        print_field!(name.name);
      },
      FactSubRecord::Fnam(fnam) => {
        print_field!(fnam.name);
      },
      FactSubRecord::Rnam(rnam) => {
        print_field!(rnam.name);
      },
      FactSubRecord::Fadt(fadt) => {
        print_field!(fadt.attribute_id_1);
        print_field!(fadt.attribute_id_2);
        print_field!(fadt.rank_data, "{:?}");
        print_field!(fadt.skill_id, "{:?}");
        print_field!(fadt.flags, "{:?}");
      },
      FactSubRecord::Anam(anam) => {
        print_field!(anam.name);
      },
      FactSubRecord::Intv(intv) => {
        print_field!(intv.value);
      },
    }
  }
}

fn trace_race(race: &RaceRecord) {
  for (sub_record_number, sub_record) in race.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      RaceSubRecord::Name(name) => {
        print_field!(name.name);
      },
      RaceSubRecord::Fnam(fnam) => {
        print_field!(fnam.name);
      },
      RaceSubRecord::Radt(radt) => {
        print_field!(radt.skill_bonuses, "{:?}");
        print_field!(radt.strength, "{:?}");
        print_field!(radt.intelligence, "{:?}");
        print_field!(radt.willpower, "{:?}");
        print_field!(radt.agility, "{:?}");
        print_field!(radt.speed, "{:?}");
        print_field!(radt.endurance, "{:?}");
        print_field!(radt.personality, "{:?}");
        print_field!(radt.luck, "{:?}");
        print_field!(radt.height, "{:?}");
        print_field!(radt.weight, "{:?}");
        print_field!(radt.flags, "{:?}");
      },
      RaceSubRecord::Npcs(npcs) => {
        print_field!(npcs.name);
      },
      RaceSubRecord::Desc(desc) => {
        print_field!(desc.description);
      },
    }
  }
}

fn trace_soun(soun: &SounRecord) {
  for (sub_record_number, sub_record) in soun.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      SounSubRecord::Name(name) => {
        print_field!(name.name);
      },
      SounSubRecord::Fnam(fnam) => {
        print_field!(fnam.name);
      },
      SounSubRecord::Data(data) => {
        print_field!(data.volume);
        print_field!(data.min_range);
        print_field!(data.max_range);
      },
    }
  }
}

fn trace_skil(skil: &SkilRecord) {
  for (sub_record_number, sub_record) in skil.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      SkilSubRecord::Indx(indx) => {
        print_field!(indx.skill_id);
      },
      SkilSubRecord::Skdt(skdt) => {
        print_field!(skdt.attribute);
        print_field!(skdt.specialization, "{:?}");
        print_field!(skdt.use_value, "{:?}");
      },
      SkilSubRecord::Desc(desc) => {
        print_field!(desc.description);
      },
    }
  }
}

fn trace_mgef(mgef: &MgefRecord) {
  for (sub_record_number, sub_record) in mgef.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      MgefSubRecord::Indx(indx) => {
        print_field!(indx.effect_id);
      },
      MgefSubRecord::Medt(medt) => {
        print_field!(medt.spell_school, "{:?}");
        print_field!(medt.base_cost);
        print_field!(medt.flags, "{:?}");
        print_field!(medt.red);
        print_field!(medt.blue);
        print_field!(medt.green);
        print_field!(medt.speed_x);
        print_field!(medt.size_x);
        print_field!(medt.size_cap);
      },
      MgefSubRecord::Itex(itex) => {
        print_field!(itex.effect_icon);
      },
      MgefSubRecord::Ptex(ptex) => {
        print_field!(ptex.particle_texture);
      },
      MgefSubRecord::Cvfx(cvfx) => {
        print_field!(cvfx.casting_visual);
      },
      MgefSubRecord::Bvfx(bvfx) => {
        print_field!(bvfx.bolt_visual);
      },
      MgefSubRecord::Hvfx(hvfx) => {
        print_field!(hvfx.hit_visual);
      },
      MgefSubRecord::Avfx(avfx) => {
        print_field!(avfx.area_visual);
      },
      MgefSubRecord::Desc(desc) => {
        print_field!(desc.description);
      },
      MgefSubRecord::Csnd(csnd) => {
        print_field!(csnd.cast_sound);
      },
      MgefSubRecord::Bsnd(bsnd) => {
        print_field!(bsnd.bolt_sound);
      },
      MgefSubRecord::Hsnd(hsnd) => {
        print_field!(hsnd.hit_sound);
      },
      MgefSubRecord::Asnd(asnd) => {
        print_field!(asnd.area_sound);
      },
    }
  }
}

fn trace_scpt(scpt: &ScptRecord, scripts: bool) {
  for (sub_record_number, sub_record) in scpt.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      ScptSubRecord::Schd(schd) => {
        print_field!(schd.name);
        print_field!(schd.num_shorts);
        print_field!(schd.num_longs);
        print_field!(schd.num_floats);
        print_field!(schd.script_data_size);
        print_field!(schd.local_var_size);
      },
      ScptSubRecord::Scvr(scvr) => {
        print_field!(scvr.variables, "{:?}");
      },
      ScptSubRecord::Scdt(scdt) => {
        print_field!(scdt.data, "{:?}");
      },
      ScptSubRecord::Sctx(sctx) => {
        if scripts {
          print_field!(sctx.text);
        }
      },
    }
  }
}

fn trace_regn(regn: &RegnRecord) {
  for (sub_record_number, sub_record) in regn.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      RegnSubRecord::Name(name) => {
        print_field!(name.name);
      },
      RegnSubRecord::Fnam(fnam) => {
        print_field!(fnam.name);
      },
      RegnSubRecord::Weat(weat) => {
        print_field!(weat.clear);
        print_field!(weat.cloudy);
        print_field!(weat.foggy);
        print_field!(weat.overcast);
        print_field!(weat.rain);
        print_field!(weat.thunder);
        print_field!(weat.ash);
        print_field!(weat.blight);
        print_field!(weat.snow);
        print_field!(weat.blizard);
      },
      RegnSubRecord::Bnam(bnam) => {
        print_field!(bnam.name);
      },
      RegnSubRecord::Cnam(cnam) => {
        print_field!(cnam.color_ref, "{:?}");
      },
      RegnSubRecord::Snam(snam) => {
        print_field!(snam.sound_name);
        print_field!(snam.chance);
      },
    }
  }
}

fn trace_bsgn(bsgn: &BsgnRecord) {
  for (sub_record_number, sub_record) in bsgn.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      BsgnSubRecord::Name(name) => {
        print_field!(name.name);
      },
      BsgnSubRecord::Fnam(fnam) => {
        print_field!(fnam.name);
      },
      BsgnSubRecord::Tnam(tnam) => {
        print_field!(tnam.name);
      },
      BsgnSubRecord::Desc(desc) => {
        print_field!(desc.description);
      },
      BsgnSubRecord::Npcs(npcs) => {
        print_field!(npcs.ability);
      },
    }
  }
}

fn trace_ltex(ltex: &LtexRecord) {
  for (sub_record_number, sub_record) in ltex.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      LtexSubRecord::Name(name) => {
        print_field!(name.name);
      },
      LtexSubRecord::Intv(intv) => {
        print_field!(intv.value);
      },
      LtexSubRecord::Data(data) => {
        print_field!(data.name);
      },
    }
  }
}

fn trace_stat(stat: &StatRecord) {
  for (sub_record_number, sub_record) in stat.sub_records.iter().enumerate() {
    println!("\t{} {}", name_to_string(sub_record.name()), sub_record_number);
    match sub_record {
      StatSubRecord::Name(name) => {
        print_field!(name.name);
      },
      StatSubRecord::Modl(modl) => {
        print_field!(modl.model);
      }
    }
  }
}

pub fn trace(esx: &Esx, scripts: bool) {
  for (record_number, record) in esx.records.iter().enumerate() {
    println!("{} {}", name_to_string(record.name()), record_number);
    match record {
      Record::Tes3(tes3) => trace_tes3(tes3),
      Record::Gmst(gmst) => trace_gmst(gmst),
      Record::Glob(glob) => trace_glob(glob),
      Record::Clas(clas) => trace_clas(clas),
      Record::Fact(fact) => trace_fact(fact),
      Record::Race(race) => trace_race(race),
      Record::Soun(soun) => trace_soun(soun),
      Record::Skil(skil) => trace_skil(skil),
      Record::Mgef(mgef) => trace_mgef(mgef),
      Record::Scpt(scpt) => trace_scpt(scpt, scripts),
      Record::Regn(regn) => trace_regn(regn),
      Record::Bsgn(bsgn) => trace_bsgn(bsgn),
      Record::Ltex(ltex) => trace_ltex(ltex),
      Record::Stat(stat) => trace_stat(stat),
      Record::Unknown(unknown) => {
        for (sub_record_number, sub_record) in unknown.sub_records.iter().enumerate() {
          println!("\t{} {}", name_to_string(sub_record.name), sub_record_number);
          println!("\t\tlength: {}", sub_record.data.len());
        }
      },
    }
  }
}
