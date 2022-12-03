(* blackbox *)
module skullfet_inverter (
`ifdef USE_POWER_PINS
    input  VSS,
    input  VDD,
`endif  // USE_POWER_PINS
    input  A,
    output Y
);
endmodule

(* blackbox *)
module skullfet_nand (
`ifdef USE_POWER_PINS
    input  VSS,
    input  VDD,
`endif  // USE_POWER_PINS
    input  A,
    input  B,
    output Y
);
endmodule
