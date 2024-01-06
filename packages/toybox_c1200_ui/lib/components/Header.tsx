import { Select, useTheme } from "@himalaya-ui/core";
import React from "react";

interface VSTHeaderProps {
  preset_value: string;
  preset_changed: (value: string | string[]) => void;
}

const VSTHeader: React.FC<VSTHeaderProps> = (props) => {
  const theme = useTheme();

  const presets = [
    { value: "Accordion", name: "Accordion" },
    { value: "AltoSax", name: "AltoSax" },
    { value: "Bandoneon", name: "Bandoneon" },
    { value: "Brass1", name: "Brass1" },
    { value: "Brass2", name: "Brass2" },
    { value: "BrassEnsemble", name: "BrassEnsemble" },
    { value: "Cello", name: "Cello" },
    { value: "ChurchOrgan", name: "ChurchOrgan" },
    { value: "Clarinet", name: "Clarinet" },
    { value: "ElecOrgan1", name: "ElecOrgan1" },
    { value: "ElecOrgan2", name: "ElecOrgan2" },
    { value: "ElecOrgan3", name: "ElecOrgan3" },
    { value: "ElecOrgan4", name: "ElecOrgan4" },
    { value: "Flute", name: "Flute" },
    { value: "FrenchHorn1", name: "FrenchHorn1" },
    { value: "FrenchHorn2", name: "FrenchHorn2" },
    { value: "Harmonica", name: "Harmonica" },
    { value: "Harp", name: "Harp" },
    { value: "Oboe", name: "Oboe" },
    { value: "Piccolo", name: "Piccolo" },
    { value: "PipeOrgan", name: "PipeOrgan" },
    { value: "Recorder", name: "Recorder" },
    { value: "ReedOrgan", name: "ReedOrgan" },
    { value: "SopranoSax", name: "SopranoSax" },
    { value: "Soundtrack", name: "Soundtrack" },
    { value: "Strings1", name: "Strings1" },
    { value: "Strings2", name: "Strings2" },
    { value: "Strings3", name: "Strings3" },
    { value: "SynPad1", name: "SynPad1" },
    { value: "SynPad2", name: "SynPad2" },
    { value: "SynPad3", name: "SynPad3" },
    { value: "TenorSax", name: "TenorSax" },
    { value: "Trumpet", name: "Trumpet" },
    { value: "Tuba", name: "Tuba" },
    { value: "Violin", name: "Violin" },
  ];

  return (
    <nav
      style={{
        backgroundColor: theme.palette.accents_0,
        borderBottom: "1px solid " + theme.palette.secondary.value,
        height: "48px",
        display: "grid",
        gridTemplateColumns: "auto 1fr auto",
        alignItems: "center",
      }}
    >
      <div style={{ justifySelf: "start" }}>Left Content</div>
      <div style={{ justifySelf: "center" }}>
        <Select
          type="success"
          initialValue={props.preset_value}
          onChange={props.preset_changed}
        >
          {presets.map((preset) => (
            <Select.Option key={preset.value} value={preset.value}>
              {preset.name}
            </Select.Option>
          ))}
        </Select>
      </div>
      <div style={{ justifySelf: "end" }}>Right Content</div>
    </nav>
  );
};

export default VSTHeader;
