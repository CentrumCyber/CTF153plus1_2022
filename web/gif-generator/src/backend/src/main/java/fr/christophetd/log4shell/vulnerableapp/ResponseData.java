package fr.christophetd.log4shell.vulnerableapp;

import java.io.Serializable;

import lombok.Data;

@Data
public class ResponseData implements Serializable {
    final String query;
    final String url; 
}
