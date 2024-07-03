<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Paired QR and RFID</name>
   <tag></tag>
   <elementGuidId>d94ac1e7-4475-4954-8d30-a88758e396fd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODRiMTc3ZmI1OTMxZjBmODdlMTU0MCIsImlhdCI6MTcxOTk3MjIxNSwiZXhwIjoxNzIwMDE1NDE1fQ.yfwxhNnlJmFMcDxen7a4z3XdD4sL1HOViBvyCCNw3FQ</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a09f2e42-6c54-4a99-9675-f863143cc77f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODRiMTc3ZmI1OTMxZjBmODdlMTU0MCIsImlhdCI6MTcxOTk3MjIxNSwiZXhwIjoxNzIwMDE1NDE1fQ.yfwxhNnlJmFMcDxen7a4z3XdD4sL1HOViBvyCCNw3FQ</value>
      <webElementGuid>822142b3-ea34-4de1-8740-a426a2120d37</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${tagUrl}${rfQr}scan?tag=C00006nmW+NI</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TAG_BASE_URL</defaultValue>
      <description></description>
      <id>55c7fe50-0c60-4fa3-b6f2-7d214b16c0db</id>
      <masked>false</masked>
      <name>tagUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rfidQrEndpoint</defaultValue>
      <description></description>
      <id>e00d8626-8db9-4397-b7ae-2b973151e9fe</id>
      <masked>false</masked>
      <name>rfQr</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>185a7f81-dd3f-4625-b01e-7eb88949a7eb</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
